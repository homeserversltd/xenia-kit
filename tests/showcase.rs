use std::fmt::Write as _;

use xenia_kit::ux::*;
use xenia_kit::{html, Fragment, Markup, Render};

fn rendered(parts: Vec<Markup>) -> Vec<u8> {
    let mut output = String::new();
    for part in parts {
        output.push_str(&part.into_string());
    }
    output.into_bytes()
}

fn fixture(name: &str) -> &'static [u8] {
    match name {
        "badge" => include_bytes!("showcase/badge.html"),
        "breadcrumbs" => include_bytes!("showcase/breadcrumbs.html"),
        "button" => include_bytes!("showcase/button.html"),
        "calendar" => include_bytes!("showcase/calendar.html"),
        "card" => include_bytes!("showcase/card.html"),
        "checkbox" => include_bytes!("showcase/checkbox.html"),
        "collapsible" => include_bytes!("showcase/collapsible.html"),
        "editable-field" => include_bytes!("showcase/editable-field.html"),
        "file-input" => include_bytes!("showcase/file-input.html"),
        "icon-button" => include_bytes!("showcase/icon-button.html"),
        "input" => include_bytes!("showcase/input.html"),
        "ip-calendar" => include_bytes!("showcase/ip-calendar.html"),
        "loading-spinner" => include_bytes!("showcase/loading-spinner.html"),
        "modal" => include_bytes!("showcase/modal.html"),
        "plus-button" => include_bytes!("showcase/plus-button.html"),
        "progress-bar" => include_bytes!("showcase/progress-bar.html"),
        "row-info-tile" => include_bytes!("showcase/row-info-tile.html"),
        "select" => include_bytes!("showcase/select.html"),
        "slider" => include_bytes!("showcase/slider.html"),
        "table" => include_bytes!("showcase/table.html"),
        "tabs" => include_bytes!("showcase/tabs.html"),
        "text-box" => include_bytes!("showcase/text-box.html"),
        "time-picker" => include_bytes!("showcase/time-picker.html"),
        "toast" => include_bytes!("showcase/toast.html"),
        "toggle" => include_bytes!("showcase/toggle.html"),
        "visibility-toggle" => include_bytes!("showcase/visibility-toggle.html"),
        other => panic!("unknown showcase fixture: {other}"),
    }
}

fn assert_fixture(name: &str, actual: Vec<u8>) {
    assert_eq!(actual, fixture(name), "{name} showcase bytes changed");
}

fn badge(text: &str, tone: Option<Tone>, size: Option<Size>) -> Badge {
    Badge {
        text: text.into(),
        tone,
        size,
        hx: Hx::default(),
    }
}

fn button(text: &str, tone: ButtonTone) -> Button {
    Button {
        text: text.into(),
        tone: Some(tone),
        ..Button::default()
    }
}

fn progress(
    kind: ProgressKind,
    label: &str,
    right: Option<&str>,
    aria_label: &str,
    value: Option<u8>,
    fill_text: Option<&str>,
    styled_labels: bool,
    default_fill: bool,
) -> ProgressBar {
    ProgressBar {
        kind,
        label: label.into(),
        right: right.map(str::to_owned),
        aria_label: aria_label.into(),
        value,
        fill_text: fill_text.map(str::to_owned),
        styled_labels,
        default_fill,
        hx: Hx::default(),
    }
}

fn tab(label: &str) -> Tab {
    Tab {
        label: label.into(),
        ..Tab::default()
    }
}

#[test]
fn badge_matches_showcase_fixture() {
    assert_fixture(
        "badge",
        rendered(
            [
                ("Primary", Some(Tone::Primary), None),
                ("Secondary", Some(Tone::Secondary), None),
                ("Success", Some(Tone::Success), None),
                ("Warning", Some(Tone::Warning), None),
                ("Danger", Some(Tone::Danger), None),
                ("Info", Some(Tone::Info), None),
                ("Small", None, Some(Size::Small)),
                ("Medium", None, Some(Size::Medium)),
                ("Large", None, Some(Size::Large)),
                ("Online", Some(Tone::Success), None),
                ("42", Some(Tone::Primary), None),
                ("Critical", Some(Tone::Danger), None),
            ]
            .into_iter()
            .map(|(text, tone, size)| badge(text, tone, size).render())
            .collect(),
        ),
    );
}

#[test]
fn breadcrumbs_matches_showcase_fixture() {
    assert_fixture(
        "breadcrumbs",
        rendered(vec![Breadcrumbs {
            label: "Test directory path".into(),
            items: vec![
                Breadcrumb {
                    text: "nas".into(),
                    path: "/mnt/nas".into(),
                    current: false,
                },
                Breadcrumb {
                    text: "media".into(),
                    path: "/mnt/nas/media".into(),
                    current: false,
                },
                Breadcrumb {
                    text: "kids".into(),
                    path: "/mnt/nas/media/kids".into(),
                    current: true,
                },
            ],
            delegated: true,
            hx: Hx::default(),
        }
        .render()]),
    );
}

#[test]
fn button_matches_showcase_fixture() {
    let mut primary_icon = button("With Icon", ButtonTone::Primary);
    primary_icon.left_icon = Some("⌘".into());
    let mut loading = button("Loading", ButtonTone::Primary);
    loading.loading = true;
    let mut primary_disabled = button("Disabled", ButtonTone::Primary);
    primary_disabled.disabled = true;
    let mut secondary_icon = button("Icon Right", ButtonTone::Secondary);
    secondary_icon.right_icon = Some("👤".into());
    let mut secondary_disabled = button("Disabled", ButtonTone::Secondary);
    secondary_disabled.disabled = true;
    let mut danger_disabled = button("Disabled", ButtonTone::Danger);
    danger_disabled.disabled = true;
    let mut warning_disabled = button("Disabled", ButtonTone::Warning);
    warning_disabled.disabled = true;
    let mut success_disabled = button("Disabled", ButtonTone::Success);
    success_disabled.disabled = true;
    assert_fixture(
        "button",
        rendered(vec![
            button("Primary Button", ButtonTone::Primary).render(),
            primary_icon.render(),
            loading.render(),
            primary_disabled.render(),
            button("Secondary Button", ButtonTone::Secondary).render(),
            secondary_icon.render(),
            secondary_disabled.render(),
            button("Danger Button", ButtonTone::Danger).render(),
            danger_disabled.render(),
            button("Warning Button", ButtonTone::Warning).render(),
            warning_disabled.render(),
            button("Success Button", ButtonTone::Success).render(),
            success_disabled.render(),
            Button {
                size: Some(Size::Small),
                ..button("Small", ButtonTone::Primary)
            }
            .render(),
            Button {
                size: Some(Size::Medium),
                ..button("Medium", ButtonTone::Primary)
            }
            .render(),
            Button {
                size: Some(Size::Large),
                ..button("Large", ButtonTone::Primary)
            }
            .render(),
        ]),
    );
}

#[test]
fn calendar_matches_showcase_fixture() {
    assert_fixture(
        "calendar",
        rendered(vec![
            Calendar {
                mode: CalendarMode::Weekdays(
                    [
                        "Monday",
                        "Tuesday",
                        "Wednesday",
                        "Thursday",
                        "Friday",
                        "Saturday",
                        "Sunday",
                    ]
                    .map(str::to_owned)
                    .to_vec(),
                ),
                hx: Hx::default(),
            }
            .render(),
            Calendar {
                mode: CalendarMode::Monthly {
                    previous: "Previous".into(),
                    next: "Next".into(),
                    previous_label: "Previous month".into(),
                    next_label: "Next month".into(),
                    month: String::new(),
                    weekdays: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
                        .map(str::to_owned)
                        .to_vec(),
                    output: String::new(),
                },
                hx: Hx::default(),
            }
            .render(),
            Calendar {
                mode: CalendarMode::Disabled("Monday".into()),
                hx: Hx::default(),
            }
            .render(),
        ]),
    );
}

#[test]
fn card_matches_showcase_fixture() {
    let action = Button {
        text: "Action".into(),
        tone: Some(ButtonTone::Primary),
        size: Some(Size::Small),
        ..Button::default()
    };
    assert_fixture(
        "card",
        rendered(vec![
            Card {
                body: CardBody::Paragraph("This is a default card with some content.".into()),
                ..Card::default()
            }
            .render(),
            Card {
                body: CardBody::Paragraph("Card body content goes here.".into()),
                header: Some("Card Header".into()),
                ..Card::default()
            }
            .render(),
            Card {
                body: CardBody::Text("Card with footer actions.".into()),
                footer: vec![action],
                body_wrapper: true,
                ..Card::default()
            }
            .render(),
            Card {
                body: CardBody::Text("Click this card to interact.".into()),
                state: Some(CardState::Clickable),
                ..Card::default()
            }
            .render(),
            Card {
                body: CardBody::Text("This card indicates an active state.".into()),
                state: Some(CardState::Active),
                ..Card::default()
            }
            .render(),
            Card {
                body: CardBody::Text("This card indicates an error state.".into()),
                state: Some(CardState::Error),
                ..Card::default()
            }
            .render(),
        ]),
    );
}

#[test]
fn card_composes_markup_and_header_badges() {
    let progress = ProgressBar {
        kind: ProgressKind::Memory,
        label: "Memory".into(),
        right: Some("42%".into()),
        aria_label: "Memory use".into(),
        value: Some(42),
        fill_text: None,
        styled_labels: false,
        default_fill: false,
        hx: Hx::default(),
    };
    let row = RowInfoTile {
        kind: RowKind::Info {
            icon: "📺".into(),
            name: "arch-tv".into(),
            detail: Some("Online".into()),
            badges: vec![],
        },
        hx: Hx::default(),
    };
    let card = Card {
        body: CardBody::Markup(html! { (progress) (row) }),
        header: Some("arch-tv".into()),
        header_badges: vec![
            Badge {
                text: "tv".into(),
                tone: Some(Tone::Info),
                ..Badge::default()
            },
            Badge {
                text: "online".into(),
                tone: Some(Tone::Success),
                ..Badge::default()
            },
        ],
        ..Card::default()
    };
    assert_eq!(
        card.render().into_string(),
        "<div class=\"ui-card\"><div class=\"ui-card__header\">arch-tv<span class=\"ui-badge ui-badge--info\">tv</span><span class=\"ui-badge ui-badge--success\">online</span></div><div class=\"ui-card__body\"><div class=\"ui-progress-bar ui-progress-bar--memory\"><div class=\"ui-progress-bar__labels\"><span>Memory</span><span>42%</span></div><div class=\"ui-progress-bar__container\" role=\"meter\" aria-label=\"Memory use\" aria-valuemin=\"0\" aria-valuemax=\"100\" aria-valuenow=\"42\"><div class=\"ui-progress-bar__fill ui-progress-bar__fill--memory\" style=\"width:42%\"></div></div></div><div class=\"row-info-tile\"><span>📺</span><strong>arch-tv</strong><span>Online</span></div></div></div>"
    );
}

#[test]
fn fragment_composes_typed_root_hx_attributes() {
    const ROOT_OPEN: &str = "<section class=\"pane\" data-pane-panel=\"x\" data-view-panel=\"x\"";
    let fragment = Fragment::new("x", html! {}).with_hx(Hx {
        get: Some("/api/tabs/x/fragment".into()),
        trigger: Some("every 15s".into()),
        swap: Some(Swap::OuterHtml),
        ..Hx::default()
    });
    let rendered = fragment.render().into_string();
    assert!(rendered.starts_with(ROOT_OPEN));
    assert_eq!(
        rendered,
        "<section class=\"pane\" data-pane-panel=\"x\" data-view-panel=\"x\" hx-get=\"/api/tabs/x/fragment\" hx-swap=\"outerHTML\" hx-trigger=\"every 15s\"></section>"
    );
}

#[test]
fn checkbox_matches_showcase_fixture() {
    let make = |label: &str, size, checked, disabled, delegated| Checkbox {
        label: label.into(),
        size,
        checked,
        disabled,
        delegated,
        hx: Hx::default(),
    };
    assert_fixture(
        "checkbox",
        rendered(vec![
            make("Checked checkbox", None, true, false, true).render(),
            make("Unchecked checkbox", None, false, false, true).render(),
            make("Another checked", None, true, false, true).render(),
            make("Small checkbox", Some(Size::Small), true, false, false).render(),
            make("Medium checkbox", Some(Size::Medium), false, false, false).render(),
            make("Large checkbox", Some(Size::Large), true, false, false).render(),
            make("Disabled unchecked", None, false, true, false).render(),
            make("Disabled checked", None, true, true, false).render(),
        ]),
    );
}

#[test]
fn collapsible_matches_showcase_fixture() {
    let make = |summary: &str, content: &str, preformatted, open, delegated| Collapsible {
        summary: summary.into(),
        content: content.into(),
        preformatted,
        open,
        delegated,
        hx: Hx::default(),
    };
    assert_fixture(
        "collapsible",
        rendered(vec![
            make(
                "Open details",
                "Composable details content.",
                false,
                false,
                false,
            )
            .render(),
            make(
                "Open by default",
                "receipt: test.component.collapsible.v1",
                true,
                true,
                false,
            )
            .render(),
            make(
                "Interactive collapsible",
                "Click summary to open or close.",
                false,
                false,
                true,
            )
            .render(),
        ]),
    );
}

#[test]
fn editable_field_matches_showcase_fixture() {
    assert_fixture(
        "editable-field",
        rendered(vec![
            EditableField {
                value: "Click to edit this text".into(),
                size: Some(EditableSize::Medium),
                ..EditableField::default()
            }
            .render(),
            EditableField {
                value: "Empty field - click to edit".into(),
                size: Some(EditableSize::Small),
                placeholder: true,
                ..EditableField::default()
            }
            .render(),
            EditableField {
                value: "Disabled field".into(),
                disabled: true,
                ..EditableField::default()
            }
            .render(),
        ]),
    );
}

#[test]
fn file_input_matches_showcase_fixture() {
    assert_fixture(
        "file-input",
        rendered(vec![
            FileInput {
                label: "Select files".into(),
                button: "Choose Files".into(),
                display: "No files selected".into(),
                input_label: "Select test files".into(),
                display_label: "Selected test files".into(),
                multiple: true,
                delegated: true,
                ..FileInput::default()
            }
            .render(),
            FileInput {
                label: "Disabled".into(),
                button: "Choose File".into(),
                display: "Selection unavailable".into(),
                display_label: "Disabled file selection".into(),
                disabled: true,
                ..FileInput::default()
            }
            .render(),
            FileInput {
                label: "File error".into(),
                button: "Choose File".into(),
                display: "Rejected file".into(),
                display_label: "Rejected file selection".into(),
                error: Some("File type is not accepted".into()),
                ..FileInput::default()
            }
            .render(),
        ]),
    );
}

#[test]
fn icon_button_matches_showcase_fixture() {
    let make = |icon, size, shape, label: &str, disabled| IconButton {
        icon,
        size,
        shape,
        label: label.into(),
        disabled,
        hx: Hx::default(),
    };
    assert_fixture(
        "icon-button",
        rendered(vec![
            make(
                Icon::Refresh,
                Size::Small,
                Shape::Square,
                "Refresh directory",
                false,
            )
            .render(),
            make(
                Icon::Sync,
                Size::Medium,
                Shape::Circle,
                "Synchronize directory",
                false,
            )
            .render(),
            make(
                Icon::Settings,
                Size::Large,
                Shape::Square,
                "Directory settings unavailable",
                true,
            )
            .render(),
        ]),
    );
}

#[test]
fn input_matches_showcase_fixture() {
    assert_fixture(
        "input",
        rendered(vec![
            Input {
                label: Some("Username".into()),
                placeholder: Some("Enter username".into()),
                kind: Some("text".into()),
                delegated: true,
                ..Input::default()
            }
            .render(),
            Input {
                label: Some("Email".into()),
                value: Some("ab".into()),
                kind: Some("text".into()),
                error: Some("Input must be at least 3 characters".into()),
                delegated: true,
                ..Input::default()
            }
            .render(),
            Input {
                placeholder: Some("Enter password".into()),
                kind: Some("password".into()),
                ..Input::default()
            }
            .render(),
            Input {
                placeholder: Some("This field is required".into()),
                required: true,
                ..Input::default()
            }
            .render(),
            Input {
                value: Some("Cannot edit".into()),
                disabled: true,
                ..Input::default()
            }
            .render(),
            Input {
                value: Some("homeserver".into()),
                display: true,
                readonly: true,
                ..Input::default()
            }
            .render(),
            Input {
                placeholder: Some("Small input".into()),
                size: Some(Size::Small),
                ..Input::default()
            }
            .render(),
            Input {
                placeholder: Some("Medium input".into()),
                size: Some(Size::Medium),
                ..Input::default()
            }
            .render(),
            Input {
                placeholder: Some("Large input".into()),
                size: Some(Size::Large),
                ..Input::default()
            }
            .render(),
        ]),
    );
}

#[test]
fn ip_calendar_matches_showcase_fixture() {
    assert_fixture(
        "ip-calendar",
        rendered(vec![IpCalendar {
            prefix: "192.168.123.".into(),
            range: "100 - 131".into(),
            previous: "Previous".into(),
            next: "Next".into(),
            previous_label: "Previous address range".into(),
            next_label: "Next address range".into(),
            grid_label: "LAN reservation addresses".into(),
            readout: "Selected address: 192.168.123.100".into(),
            hx: Hx::default(),
        }
        .render()]),
    );
}

#[test]
fn loading_spinner_matches_showcase_fixture() {
    let make = |size, label: Option<&str>, inline, hidden| LoadingSpinner {
        size,
        label: label.map(str::to_owned),
        inline,
        hidden,
        hx: Hx::default(),
    };
    assert_fixture(
        "loading-spinner",
        rendered(vec![
            make(Size::Small, Some("Small loading spinner"), false, false).render(),
            make(Size::Medium, Some("Medium loading spinner"), false, false).render(),
            make(Size::Large, Some("Large loading spinner"), false, false).render(),
            make(Size::Small, Some("Loading"), true, false).render(),
            make(Size::Small, None, true, true).render(),
            make(Size::Medium, Some("Loading network data"), false, false).render(),
            make(Size::Large, Some("Connecting to server"), false, false).render(),
        ]),
    );
}

#[test]
fn modal_matches_showcase_fixture() {
    assert_fixture(
        "modal",
        rendered(vec![Modal {
            title: "Medium modal".into(),
            title_id: "ux-modal-demo-title".into(),
            body: "Regular dialog body.".into(),
            cancel: "Cancel".into(),
            confirm: "Confirm".into(),
            close_label: "Close modal".into(),
            hidden: true,
            hx: Hx::default(),
        }
        .render()]),
    );
}

#[test]
fn plus_button_matches_showcase_fixture() {
    assert_fixture(
        "plus-button",
        rendered(vec![
            PlusButton {
                size: Some(Size::Small),
                ..PlusButton::default()
            }
            .render(),
            PlusButton {
                size: Some(Size::Medium),
                ..PlusButton::default()
            }
            .render(),
            PlusButton {
                size: Some(Size::Large),
                ..PlusButton::default()
            }
            .render(),
            PlusButton {
                secondary: true,
                ..PlusButton::default()
            }
            .render(),
            PlusButton {
                disabled: true,
                ..PlusButton::default()
            }
            .render(),
        ]),
    );
}

#[test]
fn progress_bar_matches_showcase_fixture() {
    assert_fixture(
        "progress-bar",
        rendered(vec![
            progress(
                ProgressKind::Medium,
                "Starting",
                Some("8%"),
                "Starting progress",
                Some(8),
                None,
                true,
                true,
            )
            .render(),
            progress(
                ProgressKind::Medium,
                "Halfway",
                Some("50%"),
                "Halfway progress",
                Some(50),
                Some("50%"),
                true,
                true,
            )
            .render(),
            progress(
                ProgressKind::Medium,
                "Complete",
                Some("100%"),
                "Complete progress",
                Some(100),
                Some("100%"),
                true,
                true,
            )
            .render(),
            progress(
                ProgressKind::Medium,
                "Percentage hidden",
                None,
                "Progress without visible percentage",
                Some(72),
                None,
                true,
                true,
            )
            .render(),
            progress(
                ProgressKind::Small,
                "Small",
                Some("32%"),
                "Small progress",
                Some(32),
                None,
                false,
                false,
            )
            .render(),
            progress(
                ProgressKind::Medium,
                "Medium",
                Some("58%"),
                "Medium progress",
                Some(58),
                None,
                false,
                false,
            )
            .render(),
            progress(
                ProgressKind::Large,
                "Large",
                Some("84%"),
                "Large progress",
                Some(84),
                Some("84%"),
                false,
                false,
            )
            .render(),
            progress(
                ProgressKind::Memory,
                "Memory",
                Some("68%"),
                "Memory used",
                Some(68),
                Some("68%"),
                false,
                false,
            )
            .render(),
            progress(
                ProgressKind::Swap,
                "Swap",
                Some("24%"),
                "Swap used",
                Some(24),
                None,
                false,
                false,
            )
            .render(),
            progress(
                ProgressKind::Process,
                "Process",
                Some("46%"),
                "Process capacity",
                Some(46),
                Some("46%"),
                false,
                false,
            )
            .render(),
            progress(
                ProgressKind::Disk,
                "Disk",
                Some("82%"),
                "Disk used",
                Some(82),
                Some("82%"),
                false,
                false,
            )
            .render(),
            progress(
                ProgressKind::Medium,
                "Preparing appliance update",
                Some("Working"),
                "Preparing appliance update",
                None,
                Some("Working"),
                true,
                false,
            )
            .render(),
        ]),
    );
}

#[test]
fn row_info_tile_matches_showcase_fixture() {
    assert_fixture(
        "row-info-tile",
        rendered(vec![
            RowInfoTile {
                kind: RowKind::Info {
                    icon: "📁".into(),
                    name: "Documents".into(),
                    detail: Some("Folder".into()),
                    badges: vec![],
                },
                hx: Hx::default(),
            }
            .render(),
            RowInfoTile {
                kind: RowKind::Info {
                    icon: "📄".into(),
                    name: "report.pdf".into(),
                    detail: None,
                    badges: vec![
                        badge("PDF", Some(Tone::Primary), None),
                        badge("2.3 MB", Some(Tone::Info), None),
                    ],
                },
                hx: Hx::default(),
            }
            .render(),
            RowInfoTile {
                kind: RowKind::Selected {
                    checked: true,
                    text: "📁 Old".into(),
                    badge: badge("Directory", Some(Tone::Success), None),
                },
                hx: Hx::default(),
            }
            .render(),
            RowInfoTile {
                kind: RowKind::Clickable {
                    text: "📁 Clickable Folder".into(),
                    hint: "Double-click to open".into(),
                },
                hx: Hx::default(),
            }
            .render(),
            RowInfoTile {
                kind: RowKind::Error {
                    text: "⚠️ Error File".into(),
                    badge: badge("Error", Some(Tone::Danger), None),
                },
                hx: Hx::default(),
            }
            .render(),
        ]),
    );
}

#[test]
fn select_matches_showcase_fixture() {
    let make =
        |label: &str, options: &[&str], required, disabled, delegated, error: Option<&str>| {
            Select {
                label: label.into(),
                options: options.iter().map(|value| (*value).into()).collect(),
                required,
                disabled,
                delegated,
                error: error.map(str::to_owned),
                hx: Hx::default(),
            }
        };
    assert_fixture(
        "select",
        rendered(vec![
            make(
                "Select Device to Update",
                &[
                    "Select a device",
                    "sdb - 🔒 Unlocked",
                    "sdc - 🔒 Unlocked",
                    "sdd - 🔒 Locked",
                ],
                false,
                false,
                true,
                None,
            )
            .render(),
            make(
                "Strategy",
                &[
                    "Select a strategy",
                    "Safe Key Rotation",
                    "Replace Primary Key",
                    "Add to Slot 2",
                ],
                false,
                false,
                true,
                None,
            )
            .render(),
            make(
                "Required Selection",
                &["Please select an option", "Option 1"],
                true,
                false,
                false,
                None,
            )
            .render(),
            make(
                "Select with Error",
                &["Select an option"],
                false,
                false,
                false,
                Some("This field is required"),
            )
            .render(),
            make(
                "Disabled Selection",
                &["Selected Option"],
                false,
                true,
                false,
                None,
            )
            .render(),
        ]),
    );
}

#[test]
fn slider_matches_showcase_fixture() {
    let make = |min, max, value, step, disabled, dhcp_boundary| Slider {
        min,
        max,
        value,
        step,
        disabled,
        dhcp_boundary,
        hx: Hx::default(),
    };
    assert_fixture(
        "slider",
        rendered(vec![
            make(0, 100, 50, None, false, false).render(),
            make(0, 100, 25, None, false, false).render(),
            make(0, 100, 75, Some(5), false, false).render(),
            make(0, 100, 75, None, true, false).render(),
            make(8, 247, 50, None, false, true).render(),
        ]),
    );
}

#[test]
fn table_matches_showcase_fixture() {
    let action = |text: &str, modal: &str| Button {
        text: text.into(),
        size: Some(Size::Small),
        button_type: true,
        modal_open: Some(modal.into()),
        interaction_result: Some("modal-open".into()),
        ..Button::default()
    };
    assert_fixture(
        "table",
        rendered(vec![Table {
            headers: ["Name", "Status", "Version", "Action"]
                .map(str::to_owned)
                .to_vec(),
            rows: vec![
                vec![
                    Cell::Text("Data Generator".into()),
                    Cell::Badge(badge("active", Some(Tone::Success), None)),
                    Cell::Text("1.0.0".into()),
                    Cell::Button(action("Inspect", "inspect")),
                ],
                vec![
                    Cell::Text("Health Monitor".into()),
                    Cell::Badge(badge("active", Some(Tone::Success), None)),
                    Cell::Text("1.0.0".into()),
                    Cell::Button(action("Run", "run")),
                ],
            ],
            hx: Hx::default(),
        }
        .render()]),
    );
}

#[test]
fn tabs_matches_showcase_fixture() {
    let group = |tabs| {
        Tabs {
            tabs,
            hx: Hx::default(),
        }
        .render()
    };
    let mut parts = vec![];
    for (label, size) in [
        ("Small", Size::Small),
        ("Medium", Size::Medium),
        ("Large", Size::Large),
    ] {
        parts.push(group(vec![Tab {
            label: label.into(),
            active: true,
            size: Some(size),
            ..Tab::default()
        }]));
    }
    parts.push(group(vec![
        Tab {
            label: "Active".into(),
            active: true,
            ..Tab::default()
        },
        tab("Inactive"),
        Tab {
            label: "Disabled".into(),
            disabled: true,
            ..Tab::default()
        },
        Tab {
            label: "Hidden".into(),
            hidden: true,
            ..Tab::default()
        },
        Tab {
            label: "Admin-only".into(),
            admin_only: true,
            ..Tab::default()
        },
    ]));
    parts.push(group(vec![
        Tab {
            label: "Home".into(),
            icon: Some("⌘".into()),
            active: true,
            ..Tab::default()
        },
        Tab {
            label: "Profile".into(),
            icon: Some("👤".into()),
            ..Tab::default()
        },
        Tab {
            label: "Settings".into(),
            icon: Some("⚙".into()),
            ..Tab::default()
        },
    ]));
    parts.push(group(vec![
        Tab {
            label: "Stats".into(),
            active: true,
            visibility: Some(VisibilityToggle {
                visible: true,
                label: "Hide Stats tab".into(),
                tab_column: true,
                ..VisibilityToggle::default()
            }),
            ..Tab::default()
        },
        Tab {
            label: "DHCP".into(),
            visibility: Some(VisibilityToggle {
                visible: false,
                label: "Show DHCP tab".into(),
                tab_column: true,
                ..VisibilityToggle::default()
            }),
            ..Tab::default()
        },
    ]));
    parts.push(group(vec![
        Tab {
            label: "backBlaze".into(),
            active: true,
            starred: Some(true),
            star_label: Some("backBlaze tab is starred".into()),
            ..Tab::default()
        },
        Tab {
            label: "Upload".into(),
            starred: Some(false),
            star_label: Some("Star Upload tab".into()),
            ..Tab::default()
        },
    ]));
    parts.push(group(vec![Tab {
        label: "testing".into(),
        active: true,
        visibility: Some(VisibilityToggle {
            visible: true,
            label: "Hide testing tab".into(),
            tab_column: true,
            ..VisibilityToggle::default()
        }),
        starred: Some(true),
        star_label: Some("testing tab is starred".into()),
        ..Tab::default()
    }]));
    for (depth, active, sibling) in [
        (1, "Depth 1 — document tab", "Sibling"),
        (2, "Depth 2 — pane section", "Sibling"),
        (3, "Depth 3 — category chip", "Sibling"),
    ] {
        parts.push(group(vec![
            Tab {
                label: active.into(),
                active: true,
                depth: Some(depth),
                ..Tab::default()
            },
            Tab {
                label: sibling.into(),
                depth: Some(depth),
                ..Tab::default()
            },
        ]));
    }
    parts.push(group(
        [
            "Upload",
            "Stats",
            "backBlaze",
            "DHCP",
            "Firewall",
            "Wake on LAN",
            "Linker",
            "Test",
            "testing",
        ]
        .into_iter()
        .enumerate()
        .map(|(index, label)| Tab {
            label: label.into(),
            active: index == 0,
            ..Tab::default()
        })
        .collect(),
    ));
    assert_fixture("tabs", rendered(parts));
}

#[test]
fn text_box_matches_showcase_fixture() {
    assert_fixture(
        "text-box",
        rendered(vec![
            TextBox {
                kind: TextBoxKind::Log, title: Some("System Logs".into()),
                content: "[2025-12-08 18:53:57] [INFO] Starting reinstallation of legacy installer tab: testTab\n[2025-12-08 18:55:28] [INFO] Legacy installer tab 'testTab' reinstalled successfully".into(),
                empty: false, copy_label: Some("Copy".into()), refresh_label: Some("Refresh".into()), delegated: true, hx: Hx::default(),
            }.render(),
            TextBox { kind: TextBoxKind::Code, title: None, content: "function calculateTotal(items) { return items.reduce((sum, item) => sum + (item.price * item.quantity), 0); }".into(), empty: false, copy_label: None, refresh_label: None, delegated: false, hx: Hx::default() }.render(),
            TextBox { kind: TextBoxKind::Terminal, title: None, content: "$ npm install -g serve\n$ serve -s build".into(), empty: false, copy_label: None, refresh_label: None, delegated: false, hx: Hx::default() }.render(),
            TextBox { kind: TextBoxKind::Log, title: None, content: "No log entries yet. Logs will appear here when available.".into(), empty: true, copy_label: None, refresh_label: None, delegated: false, hx: Hx::default() }.render(),
        ]),
    );
}

#[test]
fn time_picker_matches_showcase_fixture() {
    assert_fixture(
        "time-picker",
        rendered(vec![
            TimePicker {
                value: "03:00".into(),
                delegated: true,
                ..TimePicker::default()
            }
            .render(),
            TimePicker {
                value: "09:00".into(),
                disabled: true,
                ..TimePicker::default()
            }
            .render(),
        ]),
    );
}

#[test]
fn toggle_matches_showcase_fixture() {
    let make =
        |label: Option<&str>, size, checked, disabled, aria_label: Option<&str>, delegated| {
            Toggle {
                label: label.map(str::to_owned),
                size,
                checked,
                disabled,
                aria_label: aria_label.map(str::to_owned),
                delegated,
                hx: Hx::default(),
            }
        };
    assert_fixture(
        "toggle",
        rendered(vec![
            make(
                Some("Small Toggle"),
                Some(Size::Small),
                false,
                false,
                None,
                true,
            )
            .render(),
            make(
                Some("Medium Toggle"),
                Some(Size::Medium),
                true,
                false,
                None,
                true,
            )
            .render(),
            make(
                Some("Large Toggle"),
                Some(Size::Large),
                false,
                false,
                None,
                true,
            )
            .render(),
            make(
                None,
                Some(Size::Medium),
                false,
                false,
                Some("Toggle without label"),
                true,
            )
            .render(),
            make(
                Some("Disabled Off"),
                Some(Size::Medium),
                false,
                true,
                None,
                false,
            )
            .render(),
            make(
                Some("Disabled On"),
                Some(Size::Medium),
                true,
                true,
                None,
                false,
            )
            .render(),
        ]),
    );
}

#[test]
fn visibility_toggle_matches_showcase_fixture() {
    let make = |visible, size, label: &str| VisibilityToggle {
        visible,
        size,
        label: label.into(),
        tab_column: false,
        hx: Hx::default(),
    };
    assert_fixture(
        "visibility-toggle",
        rendered(vec![
            make(true, Some(Size::Small), "Hide small specimen").render(),
            make(false, Some(Size::Medium), "Show medium specimen").render(),
            make(true, Some(Size::Large), "Hide large specimen").render(),
            make(true, None, "Hide visible tab specimen").render(),
            make(false, None, "Show hidden tab specimen").render(),
            make(true, None, "Hide element specimen").render(),
            make(false, None, "Show element specimen").render(),
        ]),
    );
}

#[test]
fn toast_fixture_and_builder_keep_the_crown_marker_boundary() {
    let browser_fixture = std::str::from_utf8(fixture("toast")).expect("toast fixture is UTF-8");
    assert_eq!(
        browser_fixture.matches("data-coronatio-toast=\"\"").count(),
        5
    );
    let with_timeout = rendered(vec![Toast::new(ToastKind::Success, "Saved & <ready>.")
        .with_timeout(ToastTimeout::new(5000).expect("valid toast timeout"))
        .render()]);
    assert_eq!(
        with_timeout,
        b"<div hx-swap-oob=\"beforeend:[data-coronatio-toast-stack]\"><span hidden data-coronatio-toast-request data-toast-kind=\"success\" data-toast-timeout-ms=\"5000\">Saved &amp; &lt;ready&gt;.</span></div>"
    );
    let without_timeout = rendered(vec![
        Toast::new(ToastKind::Success, "Saved & <ready>.").render()
    ]);
    assert_eq!(
        without_timeout,
        b"<div hx-swap-oob=\"beforeend:[data-coronatio-toast-stack]\"><span hidden data-coronatio-toast-request data-toast-kind=\"success\">Saved &amp; &lt;ready&gt;.</span></div>"
    );
    let empty = rendered(vec![Toast::new(ToastKind::Success, "").render()]);
    assert_eq!(empty, b"");
}

const FIXTURE_INVENTORY: [(&str, &str, &str); 26] = [
    (
        "badge",
        "src/bands/shell/ux/showcase/badge.html",
        "b668735103beccd06dd5e0a7840a5c378c2a1747ea943c3c4e32bc1c5a386553",
    ),
    (
        "breadcrumbs",
        "src/bands/shell/ux/showcase/breadcrumbs.html",
        "a4724df67c77b22ef35faed7e7d302fe4223cdb0c94dce9e4eda0a6b0dc3a7e7",
    ),
    (
        "button",
        "src/bands/shell/ux/showcase/button.html",
        "226c0b3ca20b27d67afa157a19babd029a338bf7d1c917503135a880a313cedb",
    ),
    (
        "calendar",
        "src/bands/shell/ux/showcase/calendar.html",
        "68295ef2d4c7ec002bd4d6b7183588f0429e443a9677e87a0f3572518aaec374",
    ),
    (
        "card",
        "src/bands/shell/ux/showcase/card.html",
        "787608e2a089682c0e704fc66ba99fe70de2cfef9cd85170d3d0e68c3e216f09",
    ),
    (
        "checkbox",
        "src/bands/shell/ux/showcase/checkbox.html",
        "d80a61e19b5996c866bfbd66d1c41f7d746406353e619c183d780d8ddb58731d",
    ),
    (
        "collapsible",
        "src/bands/shell/ux/showcase/collapsible.html",
        "adc20cac4cbf34bc47bc5a22e337a171647dfa6ab2e4367f5263a99c6fab1bb4",
    ),
    (
        "editable-field",
        "src/bands/shell/ux/showcase/editable-field.html",
        "f38fe42c0acd1385c0db6f3fa659cbc7b06871c56e0be6a6226b85c939bb10cf",
    ),
    (
        "file-input",
        "src/bands/shell/ux/showcase/file-input.html",
        "bb5c849f6479e5c525fa97fe54ffa3cd29a4d6be0d3d1fdd53bd18d3e80bbeb8",
    ),
    (
        "icon-button",
        "src/bands/shell/ux/showcase/icon-button.html",
        "b79398c69c9886e42c6c1bb74c51bda06d815fc2b6240635271ceff5b5eea8b9",
    ),
    (
        "input",
        "src/bands/shell/ux/showcase/input.html",
        "a7cf26d49f4bc83699eddb5838931d7b1444254bd9fade0eb545b0515f360bb3",
    ),
    (
        "ip-calendar",
        "src/bands/shell/ux/showcase/ip-calendar.html",
        "258ac3e3b79dd3c93e26a397bd17140654ed23aac2c4f0bf9828bc02d0876e34",
    ),
    (
        "loading-spinner",
        "src/bands/shell/ux/showcase/loading-spinner.html",
        "e3957de51ec8f1c0238666669048ebac1b2303d7d976b2cb6bf5bb20896ae16a",
    ),
    (
        "modal",
        "src/bands/shell/ux/showcase/modal.html",
        "4fefd20e695b780a875247796f27b70881b238736745962d52908a9416068f21",
    ),
    (
        "plus-button",
        "src/bands/shell/ux/showcase/plus-button.html",
        "c7e91fac70a1d158c1e3fd2d7d668a279641cd21c8d7871b54f11d07910568ba",
    ),
    (
        "progress-bar",
        "src/bands/shell/ux/showcase/progress-bar.html",
        "420844ab8eed4cc4ff685f2dee0427449fb3ab9a1301658f4d661cb031c88b74",
    ),
    (
        "row-info-tile",
        "src/bands/shell/ux/showcase/row-info-tile.html",
        "6fe20ab37aa3287fbe9903915f8cf2c931213c30a9f45e9bea76addc21221dbf",
    ),
    (
        "select",
        "src/bands/shell/ux/showcase/select.html",
        "c6f30243d7e9f06a5d690d42ef3ccf1ecc08bdac73da224b4f4ffc834e0cb2c9",
    ),
    (
        "slider",
        "src/bands/shell/ux/showcase/slider.html",
        "ce18f7cf04c9f464e658eae479b67108e7e42ef92c355b466302867eefde0758",
    ),
    (
        "table",
        "src/bands/shell/ux/showcase/table.html",
        "8af101e3f164c1dbb819edc1beedfecbfd27216957fca9fb1e400f670d3b7a7c",
    ),
    (
        "tabs",
        "src/bands/shell/ux/showcase/tabs.html",
        "4c70b6a217c815a9f192303a392a7fce25149460c9c9c836c15712c33bd0c320",
    ),
    (
        "text-box",
        "src/bands/shell/ux/showcase/text-box.html",
        "d2ff3f435d274d28249cbc790ebab2c93bc0906ed89b178eb472c6897391fd67",
    ),
    (
        "time-picker",
        "src/bands/shell/ux/showcase/time-picker.html",
        "cde9d5f959d4fbc3b4da0ac62ff0f980b8c04901b281fc625f85497dc926f621",
    ),
    (
        "toast",
        "src/bands/shell/ux/showcase/toast.html",
        "2030dde518456cb2d2a4d17a13acae27fe28a032cae6cb15751240f2a31ff407",
    ),
    (
        "toggle",
        "src/bands/shell/ux/showcase/toggle.html",
        "fb0122b20c1a00169911481e3f96607952a9fb3e0c5081fd0d62a4a37119971c",
    ),
    (
        "visibility-toggle",
        "src/bands/shell/ux/showcase/visibility-toggle.html",
        "a1ad802fc2f48eb295906d3c9021e228a3f095150a1548d70d110598fad0361c",
    ),
];

fn sha256_hex(input: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut data = input.to_vec();
    let bit_len = (data.len() as u64).wrapping_mul(8);
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0);
    }
    data.extend_from_slice(&bit_len.to_be_bytes());
    let mut state = [
        0x6a09e667u32,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];
    for chunk in data.chunks_exact(64) {
        let mut words = [0u32; 64];
        for (index, word) in words[..16].iter_mut().enumerate() {
            *word = u32::from_be_bytes(chunk[index * 4..index * 4 + 4].try_into().unwrap());
        }
        for index in 16..64 {
            let s0 = words[index - 15].rotate_right(7)
                ^ words[index - 15].rotate_right(18)
                ^ (words[index - 15] >> 3);
            let s1 = words[index - 2].rotate_right(17)
                ^ words[index - 2].rotate_right(19)
                ^ (words[index - 2] >> 10);
            words[index] = words[index - 16]
                .wrapping_add(s0)
                .wrapping_add(words[index - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;
        for index in 0..64 {
            let sum1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choice = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(sum1)
                .wrapping_add(choice)
                .wrapping_add(K[index])
                .wrapping_add(words[index]);
            let sum0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = sum0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        for (slot, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *slot = slot.wrapping_add(value);
        }
    }
    let mut digest = String::with_capacity(64);
    for word in state {
        write!(&mut digest, "{word:08x}").unwrap();
    }
    digest
}

#[test]
fn showcase_fixture_inventory_has_the_frozen_digests() {
    let index: serde_json::Value =
        serde_json::from_str(include_str!("showcase/index.json")).expect("valid showcase index");
    assert_eq!(index["crown_commit"], "7cf5eeba");
    assert_eq!(
        index["face_source_sha"],
        "6bf0972afcdcf2320c45abd4d4c690bea1919b87"
    );
    let entries = index["fixtures"]
        .as_object()
        .expect("fixture inventory object");
    assert_eq!(entries.len(), FIXTURE_INVENTORY.len());
    for (name, source, expected) in FIXTURE_INVENTORY {
        let entry = &entries[name];
        assert_eq!(entry["source"], source);
        assert_eq!(entry["path"], format!("tests/showcase/{name}.html"));
        assert_eq!(entry["sha256"], expected);
        assert_eq!(
            sha256_hex(fixture(name)),
            expected,
            "{name} fixture digest changed"
        );
    }
}
