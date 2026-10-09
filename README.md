# xenia-kit

Build a household cartridge in Rust from the crown's typed UI components. The
crown supplies the library CSS, JavaScript behavior and operator skin; this crate
ships none of those assets. Builders implement `maud::Render`, so composition is
ordinary `html! { (component) }`. Text and attribute values are escaped.

The initial kit supplies Badge, Breadcrumbs, Button, Calendar, Card, Checkbox,
Collapsible, EditableField, FileInput, IconButton, Input, IpCalendar,
LoadingSpinner, Modal, PlusButton, ProgressBar, RowInfoTile, Select, Slider,
Table, Tabs (with Tab items), TextBox, TimePicker, Toggle and VisibilityToggle,
plus typed Toast requests. The visual component builders expose an `Hx` field with optional get, post,
target, swap, trigger, vals, include, confirm, indicator and disabled_elt.
Absent fields emit no HTMX attributes. Use same-origin crown proxy URLs, ordinary HTMX triggers and
delegated data seats, not JavaScript expressions or fragment initializers. A
Toast has no `Hx` field: the crown constructs the visible toast, and the guest
never builds the toast element or script. Prefer a nonempty message; an empty
message emits no toast request.

Cards compose other kit builders without exposing crown classes; typed badges go in `Card::header_badges` and builder-produced `html!` output goes in `CardBody::Markup`. A self-refreshing pane uses `Fragment::with_hx` to put typed `Hx` get, trigger, target, and swap attributes on the fragment root section.

```rust
use xenia_kit::{Fragment, html, ux::{Toast, ToastKind, ToastTimeout}};

let fragment = Fragment::new("hello-world", html! { p { "Saved" } })
    .with_toast(
        Toast::new(ToastKind::Success, "Saved & <ready>")
            .with_timeout(ToastTimeout::new(4_000)?),
    );
# Ok::<(), String>(())
```

## Posting a form

HTMX includes named controls from an enclosing form in a non-GET request. Put
this request on the submit button; the form's named fields are submitted with it.
Use typed kit controls rather than guest JavaScript or crown class names:

```rust
use xenia_kit::{html, ux::{Button, ButtonTone, Hx, Input, Select}, Form, SelectOption};

let id = "my-tab";
let form = Form {
    method_post: true,
    hx: Hx::default(),
    body: html! {
        (Select {
            label: "Deployable".into(),
            name: Some("deployable".into()),
            choices: vec![
                SelectOption {
                    value: "service-a".into(),
                    label: "Service A".into(),
                    selected: true,
                    disabled: false,
                },
                SelectOption {
                    value: "service-b".into(),
                    label: "Service B".into(),
                    selected: false,
                    disabled: false,
                },
            ],
            ..Select::default()
        })
        (Input {
            label: Some("Body ID".into()),
            name: Some("body_id".into()),
            ..Input::default()
        })
        (Button {
            tone: Some(ButtonTone::Danger),
            hx: Hx {
                post: Some(format!("/api/tabs/{id}/...")),
                target: Some("closest [data-pane-panel]".into()),
                confirm: Some("Remove this guest?".into()),
                disabled_elt: Some("this".into()),
                ..Hx::default()
            },
            ..Button::new("Remove")
        })
    },
};
```

## Five household files

    cartridge.json
    Cargo.toml
    src/main.rs
    src/fragment.rs
    src/health.rs

Cargo generates Cargo.lock. Optional `src/events.rs` can own application event
composition; `Events::router()` already mounts GET /events and POST
/events/renew. Optional `static/pack.css` may extend the crown's styles when declared in the
manifest; the crown delivers that sheet at admission. Compose crown classes
with kit builders by default. A cartridge without `style` needs no sheet. Never
link the sheet from a guest fragment. Do not add guest JavaScript, a systemd
unit, sudoers or a Harmonia module.

### cartridge.json

This is a manifest-form example, not an admitted release. The zero digest is a
specimen value: replace it with the release's actual sorted content-inventory
SHA-256 before submission. The release repository and ref are household choices.
`requires_kit` remains null until an actual kit lineage band is available; no
face or staff lineage is invented here.

```json
{
  "schema": "appliance.xenia.v1",
  "id": "hello-world",
  "display": "Hello World",
  "version": "0.1.0",
  "kind": "cartridge-process",
  "client_class": "fragment",
  "source": {"release_repo": "household/cartridge-hello-world", "ref": "v0.1.0"},
  "transport": {"declared": null},
  "proxy_route": "/api/tabs/hello-world",
  "fragment_path": "/fragment",
  "health": {"route": "/health", "command": null},
  "fault_recovery": {"on_timeout": "recovery-pane", "on_crash": "recovery-pane", "timeout_ms": 5000},
  "capabilities": [],
  "requires_kit": null,
  "content_inventory_digest": "0000000000000000000000000000000000000000000000000000000000000000"
}
```

The sample above has no `style` block, so it needs no stylesheet and uses the
crown's library and skin through the kit's markup. Compose crown classes with
the kit builders by default. To add CSS, declare an extension in `cartridge.json`
and serve it from that clone-relative path:

```json
"style": { "mode": "extend", "sheet": "static/pack.css" }
```

The crown fetches and delivers the declared sheet at admission; the guest
fragment must not carry a `<link rel="stylesheet">`. `replace` is a recognized
manifest mode but is not built yet: Caduceus admission refuses it until the crown
can honor replacement rather than silently treating it as an extension.

`Manifest::style() -> manifest::Result<Option<Style>>` reads only the raw
manifest: absent style returns `None`; a valid block returns its `Extend` or
`Replace` mode and sheet string. A present block must be an object with a
recognized string `mode` and string `sheet`; errors identify `style`,
`style.mode`, or `style.sheet`. `Manifest::from_value` performs the same check,
so malformed present style blocks fail manifest loading/startup. Unknown style
keys and other raw fields remain preserved by `raw()` and `into_raw()`.

### Cargo.toml

Select an actually published kit commit; the placeholder below is deliberately
not an invented release tag. A local checkout may instead use a path dependency.

```toml
[package]
name = "cartridge-hello-world"
version = "0.1.0"
edition = "2021"

[dependencies]
xenia-kit = { git = "https://github.com/HOMESERVERSLTD/xenia-kit", rev = "<admitted-kit-commit>" }
axum = "0.8"
maud = "0.27"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The reexported Maud `html!` macro still requires the invoking household crate to
declare `maud` directly, as shown above.

### src/main.rs

```rust
mod fragment;
mod health;

use axum::{routing::get, Router};
use xenia_kit::{Manifest, Schemas, events::Events, transport::Transport};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let schemas = Schemas::load().await?;
    let manifest = Manifest::read("cartridge.json", &schemas).await?;
    let id = manifest.id().ok_or("manifest has no service identity")?;
    let health = xenia_kit::health::Health::new(&schemas, id, true)?;
    let events = Events::new(&schemas, format!("/api/tabs/{id}/events/renew"))?;
    let transport = Transport::bind(id, manifest.declared_endpoint()).await?;
    let app = Router::new()
        .route("/fragment", get(fragment::get))
        .route("/health", get(health::get))
        .with_state(health)
        .merge(events.router());
    axum::serve(transport.listener, app).await?;
    Ok(())
}
```

### src/fragment.rs

```rust
use xenia_kit::{html, Fragment, ux::{Button, ButtonTone, Card, CardBody}};

pub async fn get() -> Fragment {
    let action = Button {
        tone: Some(ButtonTone::Primary),
        ..Button::new("Hello")
    };
    let card = Card {
        header: Some("Hello World".into()),
        body: CardBody::Paragraph("Your household cartridge is here.".into()),
        footer: vec![action],
        ..Card::default()
    };
    Fragment::new("hello-world", html! { (card) })
}
```

### src/health.rs

```rust
use axum::{extract::State, response::IntoResponse};
use xenia_kit::health::Health;

pub async fn get(State(health): State<Health>) -> impl IntoResponse {
    health.response()
}
```

## Startup and raw envelopes

Before binding, `Schemas::load()` performs HTTP GET for all three public seats:
`/api/v1/schema/appliance.xenia.v1`, `/api/v1/schema/xenia.health.v1` and
`/api/v1/schema/xenia.events.v1`. Set `XENIA_SCHEMA_BASE` to an HTTP(S) origin for
an explicit door. Without it, HTTP uses the appliance's `/run/caduceus/staff.sock`
(`CADUCEUS_STAFF_SOCKET` can name its declared socket). `Schemas::load_from(base)`
provides explicit per-process configuration without changing environment state.
A missing or foreign seat aborts startup. There is no cached or vendored fallback.

Manifest validation reads required fields, form requirements and forbidden fields,
field types, constants, enums, string patterns/lengths, numeric bounds and nested
field/item constraints from those declarations. It is not the Caduceus admission
door and does not claim to perform release inventory or source-road admission.
Unknown fields stay in `Manifest::raw()` and `into_raw()`. Optional fields are not
promoted to requirements. Health::from_value preserves additive raw fields and
validates them against its loaded seat; use it to include version or source_sha.

## HTTP and leases

`Fragment` emits a section with class pane, data-pane-panel and data-view-panel,
plus validated additional `DataSeat` attributes. Its HTTP response sets
Cache-Control: no-store and the crown's CSP. It never injects an initialization
binding. The fragment's content is maud markup; callers must compose it from the
builders and ordinary semantic content, not scripts.

`Events::new` validates all outgoing lease forms against the loaded events seat.
The mounted SSE endpoint sends pulse.open with a streamId, leaseSeconds 30 and
the configured browser-facing renewRoute. POST that route with `?streamId=...`
before 20 seconds. A successful JSON response has status renewed, route,
leaseSeconds 30 and nextRenewalBeforeSeconds 20. Expiration sends pulse.expired
with schema, streamId and status expired, then ends the stream. Renewing an
unknown or expired stream returns 410. Deadline decisions use a shared lock;
expired leases cannot be revived. Disconnect and expiration remove map entries.

`Transport::bind` accepts a literal loopback SocketAddr (also with `http://`),
or binds 127.0.0.1:0. An ephemeral bind atomically publishes its actual address
in `/var/lib/xenia/<id>/listen`; this file is a courtesy, never a declaration or
socket substitute. `bind_under` takes a scratch root for a parent hand-walk.
It rejects nonloopback addresses and unsafe path components.

`Theme::ALL` exposes the 39 shallow author-face names. `Theme::Accent.variable()`
is `var(--accent)`. `theme::lint(css)` remains the guest's CSS-variable lint:
it uses CSS tokenization, including identifier escape decoding and nested
functions, and refuses any custom property outside that face. Comments and
quoted strings are not property references. For example, `--accent` is admitted
and `--theme-primary` is not. This is a variable-boundary lint, not a general CSS
parser, sanitizer or color-policy validator.

The kit's release declaration requires the crown face band `>=0.0.1, <1.0.0`.
No publication, runtime deployment or showcase equality is implied by that
declaration.
