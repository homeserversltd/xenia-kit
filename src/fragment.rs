use axum::{
    http::{header, HeaderValue},
    response::{Html, IntoResponse, Response},
};
use maud::{html, Markup, Render};

use crate::ux::Toast;

pub const CROWN_CONTENT_SECURITY_POLICY: &str = "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; connect-src 'self'; img-src 'self' data:; base-uri 'self'; frame-ancestors 'self'; frame-src http: https:";

#[derive(Clone, Debug)]
pub struct DataSeat {
    name: String,
    value: Option<String>,
}
impl DataSeat {
    pub fn new(name: impl Into<String>, value: Option<String>) -> Result<Self, String> {
        let name = name.into();
        if !name.starts_with("data-")
            || name.len() == 5
            || !name
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            || matches!(name.as_str(), "data-pane-panel" | "data-view-panel")
        {
            return Err("expected an additional delegated data-* attribute name".into());
        }
        Ok(Self { name, value })
    }
}
pub struct Fragment {
    pub pane: String,
    pub view: String,
    pub content: Markup,
    pub data: Vec<DataSeat>,
    pub toasts: Vec<Toast>,
}
impl Fragment {
    pub fn new(id: impl Into<String>, content: Markup) -> Self {
        let pane = id.into();
        Self {
            view: pane.clone(),
            pane,
            content,
            data: Vec::new(),
            toasts: Vec::new(),
        }
    }

    pub fn with_toast(mut self, toast: Toast) -> Self {
        self.toasts.push(toast);
        self
    }
}
impl Render for Fragment {
    fn render(&self) -> Markup {
        let mut result = html! {section class="pane" data-pane-panel=(&self.pane) data-view-panel=(&self.view){(&self.content)}};
        let mut attrs = String::new();
        for seat in &self.data {
            attrs.push(' ');
            attrs.push_str(&seat.name);
            if let Some(value) = &seat.value {
                attrs.push_str("=\"");
                attrs.push_str(&html! {(value)}.0);
                attrs.push('"');
            }
        }
        if let Some(at) = result.0.find('>') {
            result.0.insert_str(at, &attrs);
        }
        html! {
            (result)
            @for toast in &self.toasts {
                (toast)
            }
        }
    }
}
impl IntoResponse for Fragment {
    fn into_response(self) -> Response {
        let mut response = Html(self.render().0).into_response();
        response
            .headers_mut()
            .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
        response.headers_mut().insert(
            header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static(CROWN_CONTENT_SECURITY_POLICY),
        );
        response
    }
}
