//! Base composites: stat tiles, a switch row, an empty state and in-place sub-tabs.
//!
//! Each composite is proven in the Test tab before any face wears it. The crown
//! library paints them (`_stat-tile.css`, `_switch.css`, `_empty-state.css`,
//! `_sub-tabs.css`); this module emits markup only, with no inline style, so it
//! also renders under a `style-src 'self'` policy. Data seats let a face bind live
//! values without writing the markup by hand.
use maud::{html, Markup, Render};

use crate::fragment::{seat, DataSeat};
use crate::ux::Button;

/// The reading at or above which a tile takes the warning state, unless set.
pub const DEFAULT_THRESHOLD: u8 = 90;

/// One reading: a label, a large tabular value, an optional 4px meter and detail.
///
/// A tile with a `percent` draws a native `meter` whose `high` mark is the
/// threshold, so the fill takes the warning colour past it, and the `flag` word
/// shows with it. A tile without a `percent` (temperature, uptime) draws no bar.
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
        let meter = self.percent.map(|value| {
            seat(
                html! {meter class="ui-stat-tile__meter" min="0" max="100" low=(threshold) high=(threshold) optimum="0" value=(value.min(100)) aria-label=(&self.label) {}},
                &self.seats.meter,
            )
        });
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
