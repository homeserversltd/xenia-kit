//! Category C showcase content: the base composites of TEST-TRANCH-09.
//!
//! Stat tiles, the switch row, the empty state and in-place sub-tabs, each with
//! every state working. No HTML element tags or class values are authored here.
use crate::{
    composite::{EmptyState, StatTile, StatTiles, Switch, SubTab, SubTabs},
    layout::{ShowcaseGrid, ShowcaseItem},
    present::Paragraph,
    ux::{Button, ButtonTone},
    Markup, Render,
};

fn item(title: &str, content: Markup) -> Markup {
    ShowcaseItem { title: title.into(), content }.render()
}

fn grid(items: impl IntoIterator<Item = Markup>) -> Markup {
    let items: Vec<Markup> = items.into_iter().collect();
    ShowcaseGrid { content: maud::html! { @for item in &items { (item) } } }.render()
}

fn tile(label: &str, reading: &str, percent: Option<u8>, detail: Option<&str>) -> StatTile {
    StatTile {
        percent,
        detail: detail.map(Into::into),
        ..StatTile::new(label, reading)
    }
}

fn system_tiles() -> StatTiles {
    StatTiles {
        label: "System".into(),
        tiles: vec![
            tile("CPU", "26%", Some(26), None),
            tile("RAM", "15%", Some(15), Some("1.7 GiB of 11.4 GiB")),
            tile("Disk /", "93%", Some(93), Some("422.6 GiB of 454.5 GiB")),
            tile("Temperature", "47.6 °C", None, None),
            tile("Uptime", "1d 14h 42m", None, None),
        ],
    }
}

fn stat_tiles() -> Markup {
    let states = StatTiles {
        label: "Tile states".into(),
        tiles: vec![
            tile("Normal", "42%", Some(42), Some("below the warning line")),
            tile("At the line", "90%", Some(90), Some("warning starts at 90")),
            tile("Full", "100%", Some(100), Some("the fill reaches the end")),
            tile("Unavailable", "—", None, Some("no reading yet")),
        ],
    };
    grid([
        item("System readings", system_tiles().render()),
        item("States", states.render()),
    ])
}

fn switches() -> Markup {
    let switch = |label: &str, description: Option<&str>, checked: bool, disabled: bool| {
        Switch {
            label: label.into(),
            description: description.map(Into::into),
            checked,
            disabled,
            ..Switch::default()
        }
        .render()
    };
    grid([
        item("On", switch("Wi-Fi radio", Some("Applies at once."), true, false)),
        item("Off", switch("Wi-Fi radio", None, false, false)),
        item("Disabled", switch("Wi-Fi radio", Some("No radio answered."), false, true)),
    ])
}

fn empty_states() -> Markup {
    grid([
        item(
            "With a next step",
            EmptyState {
                title: "No GPU on this box".into(),
                why: Some("Local models load into a graphics card's own memory.".into()),
                next: Some("Fit a GPU, and its memory slots appear here.".into()),
                action: None,
            }
            .render(),
        ),
        item(
            "With an action",
            EmptyState {
                title: "No portals yet".into(),
                why: Some("A portal opens one of your services from here.".into()),
                next: None,
                action: Some(Button {
                    tone: Some(ButtonTone::Primary),
                    button_type: true,
                    ..Button::new("Add a portal")
                }),
            }
            .render(),
        ),
        item(
            "Nothing waiting",
            EmptyState {
                title: "Nothing waiting".into(),
                why: Some("Every file this box watches is as it should be.".into()),
                ..EmptyState::default()
            }
            .render(),
        ),
    ])
}

fn sub_tabs() -> Markup {
    let text = |line: &str| Paragraph { text: line.into() }.render();
    let inner = SubTabs {
        scope: "sub-tabs-specimen-nested".into(),
        label: "Nested sub-views".into(),
        depth: 2,
        selected: None,
        tabs: vec![
            SubTab { id: "brain".into(), label: "Brain".into(), content: text("A nested group switches only its own views."), ..SubTab::default() },
            SubTab { id: "ear".into(), label: "Ear".into(), content: text("Its parent group keeps its selection."), ..SubTab::default() },
        ],
    };
    let outer = SubTabs {
        scope: "sub-tabs-specimen".into(),
        label: "Admin sub-views".into(),
        depth: 1,
        selected: None,
        tabs: vec![
            SubTab { id: "system".into(), label: "System".into(), content: system_tiles().render(), ..SubTab::default() },
            SubTab { id: "secrets".into(), label: "Secrets".into(), content: text("Write-only fields live here, in place."), ..SubTab::default() },
            SubTab { id: "updates".into(), label: "Updates".into(), count: Some("3".into()), content: text("Waiting files, each as its diff."), ..SubTab::default() },
            SubTab { id: "memory".into(), label: "Memory".into(), content: inner.render(), ..SubTab::default() },
            SubTab { id: "wifi".into(), label: "Wi-Fi".into(), hidden: true, content: text("Hidden on a box with no radio."), ..SubTab::default() },
        ],
    };
    grid([item("In-place sub-views", outer.render())])
}

/// The content of one category C showcase section, by id.
pub fn section(id: &str) -> Option<Markup> {
    Some(match id {
        "stat-tiles" => stat_tiles(),
        "switches" => switches(),
        "empty-states" => empty_states(),
        "sub-tabs" => sub_tabs(),
        _ => return None,
    })
}
