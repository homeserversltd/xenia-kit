//! Base composites: stat and status tiles, switch rows, empty states and sub-tabs.
//!
//! Each composite is proven in the Test tab before any face wears it. The crown
//! library paints them (`_stat-tile.css`, `_status-tile.css`, `_switch.css`,
//! `_empty-state.css`, `_sub-tabs.css`); this module emits markup only, with no
//! inline style, so it also renders under a `style-src 'self'` policy. Data seats
//! let a face bind live values without writing the markup by hand.
use maud::{html, Markup, Render};

use crate::fragment::{seat, DataSeat};
use crate::ux::{Button, ButtonTone, DialogId};

/// One radio option with a decorative icon and explanatory line.
#[derive(Clone, Default)]
pub struct ChoiceOption {
    pub value: String,
    pub icon: String,
    pub title: String,
    pub line: String,
    pub checked: bool,
}

/// An in-flow explained choice list backed by native radio controls.
#[derive(Clone, Default)]
pub struct Choice {
    pub name: String,
    pub label: String,
    pub options: Vec<ChoiceOption>,
    pub seats: Vec<DataSeat>,
}

impl Render for Choice {
    fn render(&self) -> Markup {
        let selected = self.options.iter().position(|option| option.checked);
        let current = selected.and_then(|i| self.options.get(i)).or_else(|| self.options.first());
        seat(html! {
            details class="ui-choice" data-ui-choice {
                summary class="ui-choice__current" data-ui-choice-current {
                    @if let Some(option) = current {
                        i class=(format!("ui-choice__icon fas {}", option.icon)) aria-hidden="true" {}
                        span class="ui-choice__title" { (&option.title) }
                    }
                }
                div class="ui-choice__list" role="radiogroup" aria-label=(&self.label) {
                    @for (index, option) in self.options.iter().enumerate() {
                        label class="ui-choice__option" {
                            input class="ui-choice__input" type="radio" name=(&self.name) value=(&option.value) checked[selected == Some(index)];
                            i class=(format!("ui-choice__icon fas {}", option.icon)) aria-hidden="true" {}
                            span class="ui-choice__title" { (&option.title) }
                            span class="ui-choice__line" { (&option.line) }
                        }
                    }
                }
            }
        }, &self.seats)
    }
}

/// A collapsible bubble whose body stays in the document flow.
#[derive(Clone, Default)]
pub struct CollapsibleBubble {
    pub title: String,
    pub line: String,
    pub open: bool,
    pub seats: Vec<DataSeat>,
    pub body: Markup,
}

impl Render for CollapsibleBubble {
    fn render(&self) -> Markup {
        seat(html! {
            details class="ui-collapsible ui-collapsible--bubble" open[self.open] {
                summary class="ui-collapsible__header" {
                    span class="ui-collapsible__title" { (&self.title) }
                    span class="ui-collapsible__line" { (&self.line) }
                    i class="ui-collapsible__icon fas fa-chevron-down" aria-hidden="true" {}
                }
                div class="ui-collapsible__content" { (&self.body) }
            }
        }, &self.seats)
    }
}

/// The reading at or above which a tile takes the warning state, unless set.
pub const DEFAULT_THRESHOLD: u8 = 90;

/// Colour band for a raised stat tile's meter or, without a meter, its reading.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Band {
    Calm,
    Busy,
    Hot,
}

impl Band {
    /// Calm below 70%, busy from 70%, and hot from 90%.
    pub fn of_percent(p: f64) -> Band {
        if p >= 90.0 {
            Band::Hot
        } else if p >= 70.0 {
            Band::Busy
        } else {
            Band::Calm
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Calm => "calm",
            Self::Busy => "busy",
            Self::Hot => "hot",
        }
    }
}

/// One reading: a label, a large tabular value, an optional 4px meter and detail.
///
/// A tile with a `percent` draws a native `meter` whose `high` mark is the
/// threshold. At or above it the `flag` word shows and a flat tile takes
/// the warning colour. Raised tiles colour the meter (or the reading when
/// there is no meter) by band instead. A tile without a `percent` draws no bar
/// unless its meter is seated, in which case it renders a valueless meter for
/// later binding.
#[derive(Clone, Default)]
pub struct StatTile {
    pub label: String,
    pub reading: String,
    pub percent: Option<u8>,
    pub threshold: Option<u8>,
    pub detail: Option<String>,
    /// The word shown beside the label in the warning state; "High" when empty.
    pub flag: Option<String>,
    pub seats: StatTileSeats,
    /// Opt in to the raised face and band colours.
    pub raised: bool,
    /// A decorative mark before the label, hidden from assistive technology.
    pub glyph: Option<String>,
    /// A unit beside the reading, outside its delegated data seat.
    pub unit: Option<String>,
    /// An explicit band for a raised tile, otherwise derived from its percent.
    pub band: Option<Band>,
}

/// Delegated data seats for a face that binds a tile's parts to live values.
#[derive(Clone, Default)]
pub struct StatTileSeats {
    pub tile: Vec<DataSeat>,
    pub reading: Vec<DataSeat>,
    pub meter: Vec<DataSeat>,
    pub detail: Vec<DataSeat>,
}

impl StatTile {
    pub fn new(label: impl Into<String>, reading: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            reading: reading.into(),
            ..Self::default()
        }
    }

    fn threshold(&self) -> u8 {
        self.threshold.unwrap_or(DEFAULT_THRESHOLD).min(100)
    }

    fn warning(&self) -> bool {
        self.percent.is_some_and(|v| v.min(100) >= self.threshold())
    }
}

impl Render for StatTile {
    fn render(&self) -> Markup {
        let threshold = self.threshold();
        let state = if self.warning() { "warning" } else { "normal" };
        let flag = self.flag.as_deref().unwrap_or("High");
        let (class, band) = if self.raised {
            let band = self
                .band
                .or_else(|| self.percent.map(|p| Band::of_percent(f64::from(p))));
            ("ui-stat-tile ui-stat-tile--raised", band.map(Band::as_str))
        } else {
            ("ui-stat-tile", None)
        };
        let reading = seat(
            html! {span class="ui-stat-tile__reading" { (&self.reading) }},
            &self.seats.reading,
        );
        let meter = if self.percent.is_some() || !self.seats.meter.is_empty() {
            Some(seat(
                html! {meter class="ui-stat-tile__meter" min="0" max="100" low=(threshold) high=(threshold) optimum="0" value=[self.percent.map(|v| v.min(100))] aria-label=(&self.label) {}},
                &self.seats.meter,
            ))
        } else { None };
        let tile = html! {
            article class=(class) data-stat-tile data-state=(state) data-band=[band] {
                div class="ui-stat-tile__head" {
                    span class="ui-stat-tile__label" {
                        @if let Some(glyph) = &self.glyph {
                            span class="ui-stat-tile__glyph" aria-hidden="true" { (glyph) }
                            " "
                        }
                        (&self.label)
                    }
                    span class="ui-stat-tile__flag" { (flag) }
                }
                @if let Some(unit) = &self.unit {
                    span class="ui-stat-tile__value" {
                        (reading)
                        span class="ui-stat-tile__unit" { (unit) }
                    }
                } @else {
                    (reading)
                }
                @if let Some(meter) = meter { (meter) }
                @if self.detail.is_some() || !self.seats.detail.is_empty() {
                    (seat(html! {span class="ui-stat-tile__detail" { (self.detail.as_deref().unwrap_or("")) }}, &self.seats.detail))
                }
            }
        };
        seat(tile, &self.seats.tile)
    }
}

/// A grid of stat tiles that fills the row and wraps at narrow widths.
#[derive(Clone, Default)]
pub struct StatTiles {
    pub label: String,
    pub tiles: Vec<StatTile>,
}

impl Render for StatTiles {
    fn render(&self) -> Markup {
        html! {
            div class="ui-stat-tiles" role="group" aria-label=(&self.label) {
                @for tile in &self.tiles { (tile) }
            }
        }
    }
}

/// The state signalled by a status mark's decorative dot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MarkTone {
    Good,
    Moving,
    Stopped,
    Idle,
}

impl MarkTone {
    fn as_str(self) -> &'static str {
        match self {
            Self::Good => "good",
            Self::Moving => "moving",
            Self::Stopped => "stopped",
            Self::Idle => "idle",
        }
    }
}

/// A state word with a decorative dot that agrees with it.
#[derive(Clone)]
pub struct StatusMark {
    pub tone: MarkTone,
    /// The visible word that names the state. A blank word omits the mark.
    pub word: String,
}

impl StatusMark {
    pub fn new(tone: MarkTone, word: impl Into<String>) -> Self {
        Self {
            tone,
            word: word.into(),
        }
    }
}

impl Render for StatusMark {
    fn render(&self) -> Markup {
        if self.word.trim().is_empty() {
            return html! {};
        }
        html! {
            span class="ui-status-mark" data-tone=(self.tone.as_str()) {
                span class="ui-status-mark__dot" aria-hidden="true" {}
                span class="ui-status-mark__word" { (&self.word) }
            }
        }
    }
}

/// A status summary, optionally opening a guest dialog in the same pane.
#[derive(Clone, Default)]
pub struct StatusTile {
    pub title: String,
    pub mark: Option<StatusMark>,
    pub reading: Option<String>,
    pub lines: Vec<String>,
    pub opens: Option<DialogId>,
}

impl StatusTile {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Self::default()
        }
    }
}

impl Render for StatusTile {
    fn render(&self) -> Markup {
        let parts = html! {
            span class="ui-status-tile__head" {
                span class="ui-status-tile__title" { (&self.title) }
                @if let Some(mark) = &self.mark { (mark) }
            }
            @if let Some(reading) = &self.reading {
                span class="ui-status-tile__reading" { (reading) }
            }
            @for line in &self.lines {
                span class="ui-status-tile__line" { (line) }
            }
        };
        match &self.opens {
            Some(id) => {
                let id = id.as_str();
                html! {
                    button class="ui-status-tile" type="button" data-status-tile
                        data-crown-dialog-open=(id) aria-haspopup="dialog" aria-controls=(id) { (parts) }
                }
            }
            None => html! { article class="ui-status-tile" data-status-tile { (parts) } },
        }
    }
}

/// A labelled, wrapping grid of status tiles with an optional wider column minimum.
#[derive(Clone, Default)]
pub struct StatusTiles {
    pub label: String,
    pub wide: bool,
    pub tiles: Vec<StatusTile>,
}

impl Render for StatusTiles {
    fn render(&self) -> Markup {
        let class = if self.wide {
            "ui-status-tiles ui-status-tiles--wide"
        } else {
            "ui-status-tiles"
        };
        html! {
            div class=(class) role="group" aria-label=(&self.label) {
                @for tile in &self.tiles { (tile) }
            }
        }
    }
}

/// A labelled on/off row that applies at once, with the control on the right.
#[derive(Clone, Default)]
pub struct Switch {
    pub label: String,
    pub description: Option<String>,
    pub name: Option<String>,
    pub checked: bool,
    pub disabled: bool,
    pub seats: Vec<DataSeat>,
}

impl Render for Switch {
    fn render(&self) -> Markup {
        let input = seat(
            html! {input class="ui-switch__input" type="checkbox" role="switch" name=[self.name.as_deref()] checked[self.checked] disabled[self.disabled];},
            &self.seats,
        );
        html! {
            label class=(if self.disabled { "ui-switch ui-switch--disabled" } else { "ui-switch" }) {
                span class="ui-switch__text" {
                    span class="ui-switch__label" { (&self.label) }
                    @if let Some(description) = &self.description {
                        span class="ui-switch__description" { (description) }
                    }
                }
                span class="ui-switch__control" {
                    (input)
                    span class="ui-switch__track" {}
                }
            }
        }
    }
}

/// What is missing, why, and the next step, with an optional single action.
#[derive(Clone, Default)]
pub struct EmptyState {
    pub title: String,
    pub why: Option<String>,
    pub next: Option<String>,
    pub action: Option<Button>,
}

impl Render for EmptyState {
    fn render(&self) -> Markup {
        html! {
            div class="ui-empty-state" role="status" {
                p class="ui-empty-state__title" { (&self.title) }
                @if let Some(why) = &self.why { p class="ui-empty-state__why" { (why) } }
                @if let Some(next) = &self.next { p class="ui-empty-state__next" { (next) } }
                @if let Some(action) = &self.action { div class="ui-empty-state__action" { (action) } }
            }
        }
    }
}

/// One guide paragraph, optionally beginning with an emphasized lead.
#[derive(Clone, Default)]
pub struct GuideLine {
    pub lead: Option<String>,
    pub text: String,
}

/// A block of plain guide paragraphs.
#[derive(Clone, Default)]
pub struct Guide {
    pub lines: Vec<GuideLine>,
}

impl Render for Guide {
    fn render(&self) -> Markup {
        html! { div class="ui-guide" {
            @for line in &self.lines {
                p { @if let Some(lead) = &line.lead { strong { (lead) } } (&line.text) }
            }
        } }
    }
}

/// One composed piece beside its name and always-visible explanation.
#[derive(Clone, Default)]
pub struct Exhibit {
    pub name: String,
    pub piece: Markup,
    pub admin: bool,
    pub words: Vec<String>,
}

impl Render for Exhibit {
    fn render(&self) -> Markup {
        html! { div class="ui-exhibit" {
            div class="ui-exhibit__piece" data-admin-mode=[self.admin.then_some("true")] { (self.piece.clone()) }
            div class="ui-exhibit__words" {
                h4 class="ui-exhibit__name" { (&self.name) }
                @for paragraph in &self.words { p { (paragraph) } }
            }
        } }
    }
}

/// One sub-view of an in-place tab group.
#[derive(Clone, Default)]
pub struct SubTab {
    pub id: String,
    pub label: String,
    /// A count shown after the label, such as the number of waiting updates.
    pub count: Option<String>,
    pub hidden: bool,
    pub content: Markup,
}

/// A nested slab tab group that switches its own sub-views in place.
///
/// The group owns one `data-tab-scope`; the host's delegated switcher selects
/// within that scope only, so groups nest to any depth. The first visible tab
/// is selected unless `selected` names another.
#[derive(Clone, Default)]
pub struct SubTabs {
    pub scope: String,
    pub label: String,
    pub depth: u8,
    pub selected: Option<String>,
    pub tabs: Vec<SubTab>,
}

impl SubTabs {
    fn selected_id(&self) -> Option<&str> {
        self.selected
            .as_deref()
            .filter(|id| self.tabs.iter().any(|tab| !tab.hidden && tab.id == *id))
            .or_else(|| self.tabs.iter().find(|t| !t.hidden).map(|t| t.id.as_str()))
    }
}

impl Render for SubTabs {
    fn render(&self) -> Markup {
        let selected = self.selected_id();
        let scope = &self.scope;
        html! {
            div class="ui-sub-tabs" data-tab-scope=(scope) data-tab-depth=(self.depth.max(1)) {
                div class="ui-tab-group" data-tab-depth=(self.depth.max(1)) role="tablist" aria-label=(&self.label) {
                    @for tab in &self.tabs {
                        @let on = selected == Some(tab.id.as_str());
                        button type="button" role="tab"
                            class=(if on { "ui-tab ui-tab--active active" } else { "ui-tab" })
                            id=(format!("{scope}-tab-{}", tab.id))
                            data-tab-id=(&tab.id)
                            aria-selected=(on.to_string())
                            aria-controls=(format!("{scope}-panel-{}", tab.id))
                            hidden[tab.hidden] {
                            span class="ui-tab__content" {
                                span class="ui-tab__label" { (&tab.label) }
                                @if let Some(count) = &tab.count {
                                    span class="ui-badge ui-badge--secondary ui-badge--small" { (count) }
                                }
                            }
                        }
                    }
                }
                @for tab in &self.tabs {
                    @let on = selected == Some(tab.id.as_str());
                    section class=(if on { "ui-sub-tabs__panel active" } else { "ui-sub-tabs__panel" })
                        role="tabpanel"
                        id=(format!("{scope}-panel-{}", tab.id))
                        aria-labelledby=(format!("{scope}-tab-{}", tab.id))
                        data-tab-panel=(&tab.id) {
                        (tab.content.clone())
                    }
                }
            }
        }
    }
}

/// The status of a write-only setting.
#[derive(Clone, Copy, Default)]
pub enum SettingState {
    #[default]
    Unset,
    Ok,
    Error,
    Saving,
}

impl SettingState {
    fn name(self) -> &'static str {
        match self {
            Self::Unset => "unset",
            Self::Ok => "ok",
            Self::Error => "error",
            Self::Saving => "saving",
        }
    }
}

/// A write-only password field; no stored value is accepted or rendered.
#[derive(Clone, Default)]
pub struct SettingField {
    pub name: String,
    pub minlength: Option<u16>,
    pub maxlength: Option<u16>,
    pub new_password: bool,
}

/// An optional POST form and its delegated root attributes.
#[derive(Clone, Default)]
pub struct SettingForm {
    pub action: String,
    pub seats: Vec<DataSeat>,
}

/// Delegated attributes for the status and feedback lines.
#[derive(Clone, Default)]
pub struct SettingRowSeats {
    pub status: Vec<DataSeat>,
    pub feedback: Vec<DataSeat>,
}

/// A setting label, status, feedback and optional write-only control.
#[derive(Clone, Default)]
pub struct SettingRow {
    pub label: String,
    pub status: Option<String>,
    pub state: SettingState,
    pub field: Option<SettingField>,
    pub action: Option<Button>,
    pub form: Option<SettingForm>,
    pub seats: SettingRowSeats,
}

impl Render for SettingRow {
    fn render(&self) -> Markup {
        let mut action = self.action.clone();
        if let Some(button) = &mut action { button.button_type = false; }
        let contents = html! {
            div class="ui-setting-row__text" {
                span class="ui-setting-row__label" { (&self.label) }
                (seat(html! { p class="ui-setting-row__status" role="status" aria-live="polite" { (self.status.as_deref().unwrap_or("")) } }, &self.seats.status))
                (seat(html! { p class="ui-setting-row__feedback" role="status" aria-live="polite" {} }, &self.seats.feedback))
            }
            div class="ui-setting-row__control" {
                @if let Some(field) = &self.field {
                    input class="ui-input ui-setting-row__field" type="password" name=(&field.name)
                        aria-label=(&self.label) required minlength=[field.minlength] maxlength=[field.maxlength]
                        autocomplete=(if field.new_password { "new-password" } else { "off" });
                }
                @if let Some(button) = &action { (button) }
            }
        };
        if let Some(form) = &self.form {
            seat(html! { form class="ui-setting-row" data-state=(self.state.name()) method="post" action=(&form.action) autocomplete="off" { (contents) } }, &form.seats)
        } else {
            html! { div class="ui-setting-row" data-state=(self.state.name()) { (contents) } }
        }
    }
}

/// A labelled list of setting rows.
#[derive(Clone, Default)]
pub struct SettingRows {
    pub label: String,
    pub rows: Vec<SettingRow>,
}

impl Render for SettingRows {
    fn render(&self) -> Markup {
        html! {
            ul class="ui-setting-rows" aria-label=(&self.label) {
                @for row in &self.rows { li class="ui-setting-rows__item" { (row) } }
            }
        }
    }
}

/// The input kind for a write-only secret tile field.
#[derive(Clone, Copy, Default)]
pub enum SecretTileFieldKind {
    #[default]
    Password,
    Text,
}

/// One required input, optionally labelled and (for text) pattern-constrained.
#[derive(Clone, Default)]
pub struct SecretTileField {
    pub field: SettingField,
    pub label: Option<String>,
    pub kind: SecretTileFieldKind,
    pub pattern: Option<String>,
}

/// Delegated data seats for the tile's status, feedback and state word.
#[derive(Clone, Default)]
pub struct SecretTileSeats {
    pub status: Vec<DataSeat>,
    pub feedback: Vec<DataSeat>,
    pub word: Vec<DataSeat>,
}

/// A tile with its own POST form. Inputs never accept or render a stored value.
#[derive(Clone, Default)]
pub struct SecretTile {
    pub label: String,
    pub about: Option<String>,
    pub status: Option<String>,
    pub state: SettingState,
    pub word: Option<String>,
    pub fields: Vec<SecretTileField>,
    /// The submit action; non-submit button and dialog modes are ignored.
    pub action: Button,
    pub form: SettingForm,
    pub seats: SecretTileSeats,
}

impl Render for SecretTile {
    fn render(&self) -> Markup {
        let word = self.word.as_deref().unwrap_or(match self.state {
            SettingState::Unset => "Not set",
            SettingState::Ok => "Set",
            SettingState::Error => "Refused",
            SettingState::Saving => "Saving",
        });
        let action = Button {
            button_type: false,
            modal_open: None,
            crown_dialog_open: None,
            modal_dismiss: false,
            ..self.action.clone()
        };
        let tile = html! {
            form class="ui-secret-tile" data-state=(self.state.name()) method="post" action=(&self.form.action) autocomplete="off" {
                div class="ui-secret-tile__head" {
                    span class="ui-secret-tile__label" { (&self.label) }
                    (seat(html! { span class="ui-secret-tile__word" role="status" { (word) } }, &self.seats.word))
                }
                @if let Some(about) = &self.about { p class="ui-secret-tile__about" { (about) } }
                (seat(html! { p class="ui-secret-tile__status" role="status" { (self.status.as_deref().unwrap_or("")) } }, &self.seats.status))
                @for control in &self.fields {
                    @let field = &control.field;
                    @let (kind, pattern, autocomplete) = match control.kind {
                        SecretTileFieldKind::Password => ("password", None, if field.new_password { "new-password" } else { "off" }),
                        SecretTileFieldKind::Text => ("text", control.pattern.as_deref(), "off"),
                    };
                    label class="ui-secret-tile__field" {
                        @if let Some(label) = &control.label { span class="ui-input-label" { (label) } }
                        input class="ui-input" type=(kind) name=(&field.name)
                            aria-label=[control.label.is_none().then_some(self.label.as_str())]
                            required minlength=[field.minlength] maxlength=[field.maxlength]
                            pattern=[pattern] autocomplete=(autocomplete);
                    }
                }
                (action)
                (seat(html! { p class="ui-secret-tile__feedback" role="status" {} }, &self.seats.feedback))
            }
        };
        seat(tile, &self.form.seats)
    }
}

/// A labelled, wrapping grid of write-only secret forms.
#[derive(Clone, Default)]
pub struct SecretTiles {
    pub label: String,
    pub tiles: Vec<SecretTile>,
}

impl Render for SecretTiles {
    fn render(&self) -> Markup {
        html! {
            div class="ui-secret-tiles" role="group" aria-label=(&self.label) {
                @for tile in &self.tiles { (tile) }
            }
        }
    }
}

/// Delegated seats on an update card and its content.
#[derive(Clone, Default)]
pub struct UpdateCardSeats {
    pub card: Vec<DataSeat>,
    pub path: Vec<DataSeat>,
    pub sentence: Vec<DataSeat>,
    pub diff: Vec<DataSeat>,
    pub feedback: Vec<DataSeat>,
}

/// An interactable file, its optional description and unified diff.
#[derive(Clone, Default)]
pub struct UpdateCard {
    pub id: String,
    pub path: String,
    pub sentence: Option<String>,
    pub diff: Option<String>,
    pub silenced: bool,
    pub seats: UpdateCardSeats,
}

fn update_action(label: &str, action: &str, primary: bool) -> Markup {
    let button = Button { button_type: true, tone: primary.then_some(ButtonTone::Primary), ..Button::new(label) };
    let attr = DataSeat::new("data-update-action", Some(action.into())).expect("fixed data seat");
    seat(button.render(), &[attr])
}

impl Render for UpdateCard {
    fn render(&self) -> Markup {
        let card = html! {
            article class="ui-update-card" data-state=(if self.silenced { "silenced" } else { "waiting" }) data-update-id=(&self.id) {
                (seat(html! { h4 class="ui-update-card__path" { (&self.path) } }, &self.seats.path))
                @if self.sentence.is_some() || !self.seats.sentence.is_empty() {
                    (seat(html! { p class="ui-update-card__sentence" { (self.sentence.as_deref().unwrap_or("")) } }, &self.seats.sentence))
                }
                @if let Some(diff) = &self.diff {
                    (seat(html! { pre class="ui-update-card__diff" {
                        @for (index, line) in diff.lines().enumerate() {
                            @if index > 0 { "\n" }
                            @let kind = if line.starts_with("+++") || line.starts_with("---") { "context" } else if line.starts_with('+') { "added" } else if line.starts_with('-') { "removed" } else { "context" };
                            span class=(format!("ui-update-card__line ui-update-card__line--{kind}")) { (line) }
                        }
                    } }, &self.seats.diff))
                } @else { p class="ui-update-card__diff-missing" { "Diff unavailable" } }
                div class="ui-update-card__actions" {
                    @if self.silenced { (update_action("Unsilence", "unsilence", false)) }
                    @else { (update_action("Silence", "silence", false)) (update_action("Yes", "run", true)) }
                }
                (seat(html! { p class="ui-update-card__feedback" role="status" aria-live="polite" {} }, &self.seats.feedback))
            }
        };
        seat(card, &self.seats.card)
    }
}

/// A labelled list, with silenced files grouped after waiting files.
#[derive(Clone, Default)]
pub struct UpdateCards {
    pub label: String,
    pub cards: Vec<UpdateCard>,
}

impl Render for UpdateCards {
    fn render(&self) -> Markup {
        html! {
            section class="ui-update-cards" aria-label=(&self.label) {
                ul class="ui-update-cards__list" {
                    @for card in self.cards.iter().filter(|c| !c.silenced) { li class="ui-update-cards__item" { (card) } }
                }
                @if self.cards.iter().any(|c| c.silenced) {
                    h3 class="ui-update-cards__group" { "Silenced" }
                    ul class="ui-update-cards__list ui-update-cards__list--silenced" {
                        @for card in self.cards.iter().filter(|c| c.silenced) { li class="ui-update-cards__item" { (card) } }
                    }
                }
            }
        }
    }
}

/// The presentation state of a usage bar.
#[derive(Clone, Copy, Default)]
pub enum UsageBarState { #[default] Filled, Empty, Total }

/// Delegated seats on a usage bar and its readings.
#[derive(Clone, Default)]
pub struct UsageBarSeats {
    pub bar: Vec<DataSeat>,
    pub name: Vec<DataSeat>,
    pub reading: Vec<DataSeat>,
    pub meter: Vec<DataSeat>,
    pub detail: Vec<DataSeat>,
}

/// One tagged native meter with an optional detail and action.
#[derive(Clone, Default)]
pub struct UsageBar {
    pub tag: String,
    pub name: String,
    pub reading: String,
    pub value: u32,
    pub max: u32,
    pub detail: Option<String>,
    pub state: UsageBarState,
    pub action: Option<Button>,
    pub seats: UsageBarSeats,
}

impl Render for UsageBar {
    fn render(&self) -> Markup {
        let max = self.max.max(1);
        let state = match self.state { UsageBarState::Filled => "filled", UsageBarState::Empty => "empty", UsageBarState::Total => "total" };
        let action = self.action.as_ref().map(|button| { let mut button = button.clone(); button.button_type = true; button });
        let bar = html! {
            li class="ui-usage-bar" data-state=(state) {
                span class="ui-usage-bar__tag" { (&self.tag) }
                div class="ui-usage-bar__body" {
                    div class="ui-usage-bar__head" {
                        (seat(html! { span class="ui-usage-bar__name" { (&self.name) } }, &self.seats.name))
                        (seat(html! { span class="ui-usage-bar__reading" { (&self.reading) } }, &self.seats.reading))
                    }
                    (seat(html! { meter class="ui-usage-bar__meter" min="0" max=(max) value=(self.value.min(max)) aria-label=(&self.tag) {} }, &self.seats.meter))
                    @if self.detail.is_some() || !self.seats.detail.is_empty() {
                        (seat(html! { span class="ui-usage-bar__detail" { (self.detail.as_deref().unwrap_or("")) } }, &self.seats.detail))
                    }
                }
                @if let Some(button) = &action { (button) }
            }
        };
        seat(bar, &self.seats.bar)
    }
}

/// A labelled list of usage bars.
#[derive(Clone, Default)]
pub struct UsageBars {
    pub label: String,
    pub bars: Vec<UsageBar>,
}

impl Render for UsageBars {
    fn render(&self) -> Markup {
        html! { ul class="ui-usage-bars" aria-label=(&self.label) { @for bar in &self.bars { (bar) } } }
    }
}
