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
Toast has no `Hx` field: the crown constructs visible notifications in response
to guest requests. Guests send a `Toast` request instead of constructing the
visible element or a script. Prefer a nonempty message; an empty message emits
no toast request.

Cards compose other kit builders without exposing crown classes; typed badges go in `Card::header_badges` and builder-produced `html!` output goes in `CardBody::Markup`. A self-refreshing pane uses `Fragment::with_hx` to put typed `Hx` get, trigger, target, and swap attributes on the fragment root section.

Build a guest-owned loading shape with `Skeleton::new("my-guest", "1")?.with_children(vec![SkeletonChild::Slot(Slot::new("summary", "summary", html! {})?.with_children(vec![SkeletonChild::Line(SkeletonLine::Medium)]))])`. `Row`, `Stack`, `Avatar`, and `Block`, plus full/medium/short lines, are typed shapes; placeholder and fill are optional, and timeouts are validated to 1-30,000 ms. Use `Slot::from_segments("details", ["section", "summary"], html! {})` to bind nested slots to their own guest path, or `Slot::ready(name, content)` for the ready `Render` form. Pending slots have no standalone `Render`; the crown owns status, `Retry`, and lifecycle, so no CSS, JavaScript, or class names are needed.

```rust
use xenia_kit::{Fragment, html, ux::{Toast, ToastKind, ToastTimeout}};

let fragment = Fragment::new("hello-world", html! { p { "Saved" } })
    .with_toast(
        Toast::new(ToastKind::Success, "Saved & <ready>")
            .with_timeout(ToastTimeout::new(4_000)?),
    );
# Ok::<(), String>(())
```

## The showcase

`xenia_kit::showcase::all()` renders the named builder specimens in the crown
specimen index order. Layout builders compose a cartridge's own panes; crown
integration is separate, and the crown owns visible toasts and scaffold lifecycle
states.

`xenia_kit::test_tab()` (also `showcase::test_tab()`) composes the complete five-panel Test tab from kit builders.

`VisibleToast` renders a visible notification; `Toast` remains the separate request builder.
`ScaffoldSlot::new(guest_id, name, state)` validates the guest and slot names.
`SlotLifecycle::Pending` renders child shapes and a status message; `Ready`
renders content and an optional update time; `Fault` renders child shapes, an
error message, and retry details. `SkeletonChild::LifecycleSlot` composes a
lifecycle slot into a skeleton. The slot itself does not start a request, but
children can contain requesting slots.

## Layout parts

`layout::TabChoice` declares a labelled, selected or unselected tab.
`layout::TabDepth` selects document, panel or category nesting.
`layout::TabStrip` renders a delegated tab group at a chosen nesting depth.
`layout::Tablet` combines a scoped tab strip and panels.
`layout::Showcase` combines category tabs and category panels.
`layout::Panel` renders a named tab panel around composed content.
`layout::ShowcaseGrid` arranges showcase items.
`layout::ShowcaseItem` labels one composed item.
`layout::ShowcaseCategory` groups a named category and its content.
`layout::SubpanelHeader` renders a subpanel's labels and optional trailing content.
`layout::ToastSpawn` renders a host-delegated notification button.
`layout::PortalCard` renders a service face with an image and state.
`layout::PortalGrid` arranges service portals and their add control.
`layout::ServiceNotice` renders a script-managed service notice.
`layout::ConfigPattern` renders a labelled configuration card.
`layout::HealthReadout` renders a labelled service-health state.
`layout::AnimationSpecimen` wraps a motion stage and an optional play control.
`layout::ServicePortal` wraps a portal card with visibility and service identity.
`layout::ServiceAction` renders an individual service control.
`layout::ServiceActions` groups service controls into rows.
`layout::StatusDot` renders an up, down or unknown indicator.
`layout::ResolverOption` describes an upstream DNS choice.
`layout::UpstreamResolvers` composes provider, custom and encrypted-DNS controls.
`layout::HealthViewport` wraps a labelled health view and its contents.
`layout::AnimationBand` groups named animation content.
`layout::MotionStage` renders a labelled motion stage.
`layout::StillnessControl` renders a delegated stop-motion control.
`layout::AddPortalCard` renders the delegated portal-creation control.
`layout::ResolverSpecimen` supplies the delegated resolver scope.
`layout::HealthDot` renders a health-summary indicator.
`motion::MotionAtomSpecimen` composes a labelled motion atom and play control.
`motion::MotionDetailStage` selects a detailed composed stage.
`motion::StableCardStage` provides a fixed hit area with a raised face.
`motion::MotionLifecycleStage` renders a phase readout and lifecycle control.
`motion::ReducedMotionGroup` pairs ordinary and reduced-motion examples.
`upload::DirectoryCrumb` describes a directory breadcrumb.
`upload::DirectoryLocation` renders breadcrumbs and selected-directory readout.
`upload::DirectoryEntry` describes a selectable tree row.
`upload::DirectoryTree` renders selectable directory rows.
`upload::DirectoryBrowser` combines directory controls, location and tree.
`upload::UploadControls` renders file selection and upload controls.
`upload::UploadPercent` represents a bounded progress value.
`upload::UploadProgress` renders a single status row.
`upload::UploadProgressList` groups upload status rows.
`upload::UploadPack` combines status rows, directory browser and file controls.
`status::ReadbackEntry` describes a configuration label and value.
`status::ReadbackSection` renders labelled configuration entries.
`status::ConfigChoice` describes a selector choice.
`status::ConfigEditor` renders an editable configuration form body.
`status::SettingsRow` describes a four-column settings row.
`status::SettingsTable` renders a typed settings grid.
`status::ViewLayout` arranges configuration, health or animation cards.
`status::HealthDependency` describes one dependency state.
`status::HealthSummary` renders a score and dependency list.
`status::TimelineEntry` describes a diagnostic event.
`status::HealthTimeline` renders ordered diagnostic events.
`status::ResultCount` describes a diagnostic count.
`status::HealthResults` renders diagnostic counts.
`status::HealthOutput` renders an escaped diagnostic readout.
`status::HealthCheck` renders the delegated health-check action.
`present::Text` wraps composed inline content in a semantic text element.
`present::Paragraph` renders escaped paragraph text.
`present::ShowcaseRow` arranges controls, optionally as a modal catalog.
`present::ShowcaseBox` wraps an unlabelled showcase item.
`present::ShowcaseArticle` renders a labelled showcase article.
`present::List` renders ordered or unordered content entries.
`present::CatalogGrid` renders a marked visibility, spinner, toast or upload catalog.
`present::SliderValue` renders a delegated slider readout.
`specimen::DriveChoice` describes one grouped drive checkbox.
`specimen::DeviceChoices` renders a device with grouped checkboxes.
`specimen::VisibilityCard` renders a delegated visible or dimmed element.
`specimen::VisibilityTab` describes one tab in a visibility comparison.
`specimen::VisibilityTabSet` renders visibility and admin-only tabs.
`specimen::PortalRemoveSimulator` renders a portal-removal specimen.
`specimen::VisibilityNote` renders a marked explanation of a visibility update.
`specimen::LoadingFrame` renders a delegated loading-state frame.
`specimen::LoadingView` renders a full-view loading state.
`specimen::LoadingButton` renders a busy action with a spinner.
`specimen::ScaffoldGrid` scopes scaffold examples to a guest and version.
`specimen::ProgressGroup` renders a marked progress specimen.
`specimen::BreadcrumbReadout` renders a delegated path readout and reset.
`specimen::FileSelectionReadout` renders a selection readout and action.
`specimen::SliderLabels` renders two slider endpoint labels.
`specimen::AddressBoundaryReadouts` renders delegated address-boundary readouts.
`specimen::CalendarReadout` renders a weekly calendar readout.
`specimen::TimeReadout` renders a time selection readout.
`specimen::MonthlyCalendarItem` wraps a marked monthly calendar.
`specimen::AddressCalendarItem` wraps a marked network address calendar.

The associated choices for these builders are:
`layout::PanelKind`: Basic, Services, Configuration, Health, Animations.
`layout::ToastSpawnKind`: Info, Success, Warning, Error, Stack.
`layout::PortalState`: Up, Down, Partial, Unknown.
`layout::ConfigKind`: Readback, Form, Table.
`layout::HealthState`: Healthy, Partial, Down, Unknown.
`layout::HealthViewKind`: Summary, Services, Diagnostics.
`layout::MotionKind`: Plain, Fade, Slide, Scale, Color, Height, ReducedSlide.
`motion::MotionAtom`: Fade, Slide, Scale, Color, Height.
`motion::MotionPhase`: Rest, Enter, Hold, Exit.
`present::TextKind`: Paragraph, MutedParagraph, Strong, Code, Span, Div, Heading3, Heading4, Heading5.
`present::CatalogKind`: Visibility, Spinner, Toast, UploadLibrary.
`specimen::VisibilityNoteKind`: ElementsChanged, LiveMorph.
`specimen::ProgressGroupKind`: Basic, Sizes, Variants, Indeterminate.
`status::ViewLayoutKind`: Configuration, Health, Animation.
`upload::DirectoryExpansion`: Leaf, Expanded, Collapsed.
`upload::UploadStatus`: Pending, Uploading, Completed, Error.
`upload::UploadProgressPlacement`: DomainPack, ProgressPanel.

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

This example pins an existing kit commit. Update the revision to a later admitted
commit to use newer builders. A local checkout may instead use a path dependency.

```toml
[package]
name = "cartridge-hello-world"
version = "0.1.0"
edition = "2021"

[dependencies]
xenia-kit = { git = "https://github.com/homeserversltd/xenia-kit", rev = "3294ebd9ddf7122c6d813faa1639074b2533f4d5" }
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
