//! Base composites: stat tiles, a switch row, an empty state and in-place sub-tabs.
//!
//! Each composite is proven in the Test tab before any face wears it. The crown
//! library paints them (`_stat-tile.css`, `_switch.css`, `_empty-state.css`,
//! `_sub-tabs.css`); this module emits markup only, with no inline style, so it
//! also renders under a `style-src 'self'` policy. Data seats let a face bind live
//! values without writing the markup by hand.
use maud::{html, Markup, Render};

use crate::fragment::{seat, DataSeat};
use crate::ux::{Button, ButtonTone};

/// The reading at or above which a tile takes the warning state, unless set.
pub const DEFAULT_THRESHOLD: u8 = 90;

/// One reading: a label, a large tabular value, an optional 4px meter and detail.
///
/// A tile with a `percent` draws a native `meter` whose `high` mark is the
/// threshold, so the fill takes the warning colour past it, and the `flag` word
/// shows with it. A tile without a `percent` draws no bar unless its meter is
/// seated, in which case it renders a valueless meter for later binding.
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
        let meter = if self.percent.is_some() || !self.seats.meter.is_empty() {
            Some(seat(
                html! {meter class="ui-stat-tile__meter" min="0" max="100" low=(threshold) high=(threshold) optimum="0" value=[self.percent.map(|v| v.min(100))] aria-label=(&self.label) {}},
                &self.seats.meter,
            ))
        } else { None };
        let tile = html! {
            article class="ui-stat-tile" data-stat-tile data-state=(state) {
                div class="ui-stat-tile__head" {
                    span class="ui-stat-tile__label" { (&self.label) }
                    span class="ui-stat-tile__flag" { (flag) }
                }
                (seat(html! {span class="ui-stat-tile__reading" { (&self.reading) }}, &self.seats.reading))
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

/// An underline tab group that switches its own sub-views in place.
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
                div class="ui-tab-group ui-tab-group--underline" role="tablist" aria-label=(&self.label) {
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
