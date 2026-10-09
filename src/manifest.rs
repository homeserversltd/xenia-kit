//! Startup-only public-seat loading. The raw envelope remains authoritative.
use serde_json::Value;
use std::{path::Path, sync::Arc, time::Duration};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Seat {
    id: String,
    raw: Arc<Value>,
}
impl Seat {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn declaration(&self) -> &Value {
        &self.raw
    }
    pub fn validate(&self, raw: &Value, form: Option<&str>) -> Result<()> {
        if raw.get("schema").and_then(Value::as_str) != Some(self.id()) {
            return Err(format!(
                "foreign schema: expected {}, received {}",
                self.id,
                raw.get("schema").unwrap_or(&Value::Null)
            )
            .into());
        }
        validate_node(raw, &self.raw, "$")?;
        if let Some(form) = form {
            let rule = self
                .raw
                .get("forms")
                .and_then(|v| v.get(form))
                .ok_or_else(|| format!("seat desync: {} has no form {form}", self.id))?;
            validate_node(raw, rule, "$")?;
        }
        Ok(())
    }
}

/// Construct once before mounting any routes. No disk or embedded seat fallback.
#[derive(Clone, Debug)]
pub struct Schemas {
    pub manifest: Seat,
    pub health: Seat,
    pub events: Seat,
}
impl Schemas {
    pub async fn load() -> Result<Self> {
        match std::env::var("XENIA_SCHEMA_BASE") {
            Ok(base) => Self::load_from(&base).await,
            Err(std::env::VarError::NotPresent) => {
                let socket = std::env::var_os("CADUCEUS_STAFF_SOCKET")
                    .unwrap_or_else(|| "/run/caduceus/staff.sock".into());
                let client = reqwest::Client::builder()
                    .unix_socket(std::path::PathBuf::from(socket))
                    .timeout(Duration::from_secs(10))
                    .build()?;
                Self::load_using(&client, "http://caduceus.local").await
            }
            Err(e) => Err(e.into()),
        }
    }
    pub async fn load_from(base: &str) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        Self::load_using(&client, base).await
    }
    async fn load_using(client: &reqwest::Client, base: &str) -> Result<Self> {
        let base = reqwest::Url::parse(base)?;
        if !matches!(base.scheme(), "http" | "https")
            || !base.username().is_empty()
            || base.password().is_some()
            || base.query().is_some()
            || base.fragment().is_some()
        {
            return Err(
                "schema base must be an HTTP origin without credentials, query or fragment".into(),
            );
        }
        async fn fetch(client: &reqwest::Client, base: &reqwest::Url, id: &str) -> Result<Seat> {
            let url = base.join(&format!("/api/v1/schema/{id}"))?;
            let raw: Value = client
                .get(url)
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?;
            if raw.get("schema").and_then(Value::as_str) != Some(id)
                || !raw.get("fields").is_some_and(Value::is_object)
                || !raw.get("required").is_some_and(Value::is_array)
            {
                return Err(format!(
                    "seat desync: requested {id}; received an incompatible declaration"
                )
                .into());
            }
            Ok(Seat {
                id: id.into(),
                raw: Arc::new(raw),
            })
        }
        let (manifest, health, events) = tokio::try_join!(
            fetch(client, &base, "appliance.xenia.v1"),
            fetch(client, &base, "xenia.health.v1"),
            fetch(client, &base, "xenia.events.v1")
        )?;
        Ok(Self {
            manifest,
            health,
            events,
        })
    }
}

fn type_matches(value: &Value, kind: &str) -> bool {
    match kind {
        "null" => value.is_null(),
        "string" => value.is_string(),
        "object" => value.is_object(),
        "array" => value.is_array(),
        "boolean" => value.is_boolean(),
        "integer" => value.is_i64() || value.is_u64(),
        "number" => value.is_number(),
        _ => false,
    }
}
fn validate_node(value: &Value, rule: &Value, path: &str) -> Result<()> {
    if let Some(types) = rule.get("type") {
        let good = match types {
            Value::String(t) => type_matches(value, t),
            Value::Array(ts) => ts
                .iter()
                .filter_map(Value::as_str)
                .any(|t| type_matches(value, t)),
            _ => false,
        };
        if !good {
            return Err(format!("{path}: seat type constraint {types}").into());
        }
    }
    if let Some(expected) = rule.get("const") {
        if value != expected {
            return Err(format!("{path}: seat const constraint").into());
        }
    }
    if let Some(choices) = rule.get("enum").and_then(Value::as_array) {
        if !choices.contains(value) {
            return Err(format!("{path}: seat enum constraint").into());
        }
    }
    if let Some(text) = value.as_str() {
        if let Some(pattern) = rule.get("pattern").and_then(Value::as_str) {
            if !regex::Regex::new(pattern)?.is_match(text) {
                return Err(format!("{path}: seat pattern constraint").into());
            }
        }
        for (key, too_short) in [("minLength", true), ("maxLength", false)] {
            if let Some(limit) = rule.get(key).and_then(Value::as_u64) {
                let len = text.chars().count() as u64;
                if if too_short { len < limit } else { len > limit } {
                    return Err(format!("{path}: seat {key} constraint").into());
                }
            }
        }
    }
    if let Some(n) = value.as_f64() {
        for (key, below) in [("minimum", true), ("maximum", false)] {
            if let Some(bound) = rule.get(key).and_then(Value::as_f64) {
                if if below { n < bound } else { n > bound } {
                    return Err(format!("{path}: seat {key} constraint").into());
                }
            }
        }
    }
    if let Some(object) = value.as_object() {
        for (key, forbidden) in [("required", false), ("forbidden", true)] {
            if let Some(names) = rule.get(key).and_then(Value::as_array) {
                for name in names {
                    let name = name.as_str().ok_or("seat desync: non-string field name")?;
                    if object.contains_key(name) == forbidden {
                        return Err(format!("{path}.{name}: seat {key} constraint").into());
                    }
                }
            }
        }
        if let Some(fields) = rule.get("fields").and_then(Value::as_object) {
            for (name, child) in object {
                if let Some(child_rule) = fields.get(name) {
                    validate_node(child, child_rule, &format!("{path}.{name}"))?;
                }
            }
        }
    }
    if let (Some(items), Some(item_rule)) = (value.as_array(), rule.get("items")) {
        for (i, item) in items.iter().enumerate() {
            validate_node(item, item_rule, &format!("{path}[{i}]"))?;
        }
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Style {
    pub mode: StyleMode,
    pub sheet: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StyleMode {
    Extend,
    Replace,
}

#[derive(Clone, Debug)]
pub struct Manifest {
    raw: Value,
}
impl Manifest {
    pub async fn read(path: impl AsRef<Path>, schemas: &Schemas) -> Result<Self> {
        Self::from_value(
            serde_json::from_slice(&tokio::fs::read(path).await?)?,
            schemas,
        )
    }
    pub fn from_value(raw: Value, schemas: &Schemas) -> Result<Self> {
        schemas.manifest.validate(&raw, Some("manifest"))?;
        let manifest = Self { raw };
        manifest.style()?;
        Ok(manifest)
    }
    pub fn raw(&self) -> &Value {
        &self.raw
    }
    pub fn style(&self) -> Result<Option<Style>> {
        let Some(value) = self.raw.get("style") else {
            return Ok(None);
        };
        let object = value.as_object().ok_or("style: expected an object")?;
        let mode = object
            .get("mode")
            .and_then(Value::as_str)
            .ok_or("style.mode: expected a string")?;
        let mode = match mode {
            "extend" => StyleMode::Extend,
            "replace" => StyleMode::Replace,
            _ => return Err("style.mode: expected 'extend' or 'replace'".into()),
        };
        let sheet = object
            .get("sheet")
            .and_then(Value::as_str)
            .ok_or("style.sheet: expected a string")?
            .to_owned();
        Ok(Some(Style { mode, sheet }))
    }
    pub fn into_raw(self) -> Value {
        self.raw
    }
    pub fn id(&self) -> Option<&str> {
        self.raw.get("id").and_then(Value::as_str)
    }
    pub fn declared_endpoint(&self) -> Option<&str> {
        self.raw
            .pointer("/transport/declared")
            .and_then(Value::as_str)
    }
}
