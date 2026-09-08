//! Health receipts are validated against the startup-loaded public seat.
use crate::manifest::{Result, Schemas};
use axum::Json;
use serde_json::{json, Value};

#[derive(Clone, Debug)]
pub struct Health {
    raw: Value,
}
impl Health {
    pub fn new(schemas: &Schemas, service: &str, ok: bool) -> Result<Self> {
        Self::from_value(
            schemas,
            json!({"schema": schemas.health.id(), "service":service,"ok":ok}),
        )
    }
    pub fn from_value(schemas: &Schemas, raw: Value) -> Result<Self> {
        schemas.health.validate(&raw, None)?;
        Ok(Self { raw })
    }
    pub fn raw(&self) -> &Value {
        &self.raw
    }
    pub fn response(self) -> Json<Value> {
        Json(self.raw)
    }
}
