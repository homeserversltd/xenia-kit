//! Mountable SSE leases. Renew and expiry serialize at one deadline lock.
use crate::manifest::{Result, Schemas, Seat};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{
        sse::{Event, Sse},
        IntoResponse, Response,
    },
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{sync::mpsc, time::Instant};

const LEASE: Duration = Duration::from_secs(30);
#[derive(Clone)]
pub struct Events {
    inner: Arc<Inner>,
}
struct Inner {
    seat: Seat,
    renew_route: String,
    leases: Mutex<HashMap<String, Instant>>,
}
struct Cleanup {
    inner: Arc<Inner>,
    id: String,
}
impl Drop for Cleanup {
    fn drop(&mut self) {
        self.inner
            .leases
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(&self.id);
    }
}
impl Events {
    /// `renew_route` is the browser-facing crown proxy route, not the local mount.
    pub fn new(schemas: &Schemas, renew_route: impl Into<String>) -> Result<Self> {
        let renew_route = renew_route.into();
        if !renew_route.starts_with('/')
            || renew_route.starts_with("//")
            || renew_route.contains(['\r', '\n', '?', '#'])
        {
            return Err("renew route must be a same-origin absolute path".into());
        }
        let this = Self {
            inner: Arc::new(Inner {
                seat: schemas.events.clone(),
                renew_route,
                leases: Mutex::new(HashMap::new()),
            }),
        };
        // Check all emitted forms at startup, before any connection is admitted.
        this.open_value("startup")?;
        this.renewal_value("startup")?;
        this.expired_value("startup")?;
        Ok(this)
    }
    pub fn router(&self) -> Router {
        Router::new()
            .route("/events", get(open))
            .route("/events/renew", post(renew))
            .with_state(self.clone())
    }
    fn checked(&self, form: &str, raw: Value) -> Result<Value> {
        self.inner.seat.validate(&raw, Some(form))?;
        Ok(raw)
    }
    fn open_value(&self, id: &str) -> Result<Value> {
        self.checked("open",json!({"schema":self.inner.seat.id(),"streamId":id,"leaseSeconds":30,"renewRoute":self.inner.renew_route}))
    }
    fn renewal_value(&self, id: &str) -> Result<Value> {
        self.checked("renewal",json!({"schema":self.inner.seat.id(),"streamId":id,"route":self.inner.renew_route,"leaseSeconds":30,"status":"renewed","nextRenewalBeforeSeconds":20}))
    }
    fn expired_value(&self, id: &str) -> Result<Value> {
        self.checked(
            "expired",
            json!({"schema":self.inner.seat.id(),"streamId":id,"status":"expired"}),
        )
    }
    pub fn active_leases(&self) -> usize {
        let mut map = self.inner.leases.lock().unwrap_or_else(|e| e.into_inner());
        map.retain(|_, deadline| *deadline > Instant::now());
        map.len()
    }
}
fn failure(error: impl ToString) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
}
async fn open(State(events): State<Events>) -> Response {
    let id = uuid::Uuid::new_v4().to_string();
    let opened = match events.open_value(&id) {
        Ok(v) => v,
        Err(e) => return failure(e),
    };
    let expired = match events.expired_value(&id) {
        Ok(v) => v,
        Err(e) => return failure(e),
    };
    let (sender, mut receiver) = mpsc::channel::<Event>(2);
    let deadline = Instant::now() + LEASE;
    {
        let mut map = events
            .inner
            .leases
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        map.retain(|_, d| *d > Instant::now());
        map.insert(id.clone(), deadline);
    }
    let cleanup = Cleanup {
        inner: events.inner.clone(),
        id: id.clone(),
    };
    // The task owns cleanup before its first poll, including cancellation.
    tokio::spawn(async move {
        let _cleanup = cleanup;
        let event = Event::default()
            .event("pulse.open")
            .data(opened.to_string());
        if sender.send(event).await.is_err() {
            return;
        }
        loop {
            let deadline = {
                events
                    .inner
                    .leases
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .get(&id)
                    .copied()
            };
            let Some(deadline) = deadline else {
                break;
            };
            tokio::select! {
                _=sender.closed()=>return,
                _=tokio::time::sleep_until(deadline)=>{}
            }
            let expired_now = {
                let mut map = events
                    .inner
                    .leases
                    .lock()
                    .unwrap_or_else(|e| e.into_inner());
                match map.get(&id) {
                    Some(d) if *d > Instant::now() => false,
                    _ => {
                        map.remove(&id);
                        true
                    }
                }
            };
            if expired_now {
                break;
            }
        }
        // Map removal precedes delivery, even when the peer stops reading.
        let _ = sender.try_send(
            Event::default()
                .event("pulse.expired")
                .data(expired.to_string()),
        );
    });
    let stream = async_stream::stream! {while let Some(event)=receiver.recv().await {yield Ok::<Event,std::convert::Infallible>(event);}};
    Sse::new(stream).into_response()
}
#[derive(Deserialize)]
struct Renewal {
    #[serde(rename = "streamId")]
    stream_id: String,
}
async fn renew(State(events): State<Events>, Query(query): Query<Renewal>) -> Response {
    let raw = match events.renewal_value(&query.stream_id) {
        Ok(v) => v,
        Err(e) => return failure(e),
    };
    let renewed = {
        let mut map = events
            .inner
            .leases
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let now = Instant::now();
        map.retain(|_, d| *d > now);
        match map.get_mut(&query.stream_id) {
            Some(deadline) => {
                *deadline = now + LEASE;
                true
            }
            None => false,
        }
    };
    if renewed {
        Json(raw).into_response()
    } else {
        StatusCode::GONE.into_response()
    }
}
