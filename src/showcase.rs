//! Builder-generated examples of the crown's primitive specimens.
//!
//! Specimens use the kit's public builders, including visible lifecycle states.
use crate::ux::*;
use crate::{html, Markup, Render};
use maud::PreEscaped;

fn rendered(parts: Vec<Markup>) -> Markup {
    PreEscaped(parts.into_iter().map(Markup::into_string).collect())
}

fn sample_badge(text: &str, tone: Option<Tone>, size: Option<Size>) -> Badge {
    Badge {
        text: text.into(),
        tone,
        size,
        hx: Hx::default(),
    }
}

fn sample_button(text: &str, tone: ButtonTone) -> Button {
    Button {
        text: text.into(),
        tone: Some(tone),
        ..Button::default()
    }
}

fn sample_progress(
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

fn sample_tab(label: &str) -> Tab {
    Tab {
        label: label.into(),
        ..Tab::default()
    }
}

/// Render the badge specimens.
pub fn badge() -> Markup {
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
        .map(|(text, tone, size)| sample_badge(text, tone, size).render())
        .collect(),
    )
}

/// Render the breadcrumbs specimens.
pub fn breadcrumbs() -> Markup {
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
    .render()])
}

/// Render the button specimens.
pub fn button() -> Markup {
    let mut primary_icon = sample_button("With Icon", ButtonTone::Primary);
    primary_icon.left_icon = Some("⌘".into());
    let mut loading = sample_button("Loading", ButtonTone::Primary);
    loading.loading = true;
    let mut primary_disabled = sample_button("Disabled", ButtonTone::Primary);
    primary_disabled.disabled = true;
    let mut secondary_icon = sample_button("Icon Right", ButtonTone::Secondary);
    secondary_icon.right_icon = Some("👤".into());
    let mut secondary_disabled = sample_button("Disabled", ButtonTone::Secondary);
    secondary_disabled.disabled = true;
    let mut danger_disabled = sample_button("Disabled", ButtonTone::Danger);
    danger_disabled.disabled = true;
    let mut warning_disabled = sample_button("Disabled", ButtonTone::Warning);
    warning_disabled.disabled = true;
    let mut success_disabled = sample_button("Disabled", ButtonTone::Success);
    success_disabled.disabled = true;
    rendered(vec![
        sample_button("Primary Button", ButtonTone::Primary).render(),
        primary_icon.render(),
        loading.render(),
        primary_disabled.render(),
        sample_button("Secondary Button", ButtonTone::Secondary).render(),
        secondary_icon.render(),
        secondary_disabled.render(),
        sample_button("Danger Button", ButtonTone::Danger).render(),
        danger_disabled.render(),
        sample_button("Warning Button", ButtonTone::Warning).render(),
        warning_disabled.render(),
        sample_button("Success Button", ButtonTone::Success).render(),
        success_disabled.render(),
        Button {
            size: Some(Size::Small),
            ..sample_button("Small", ButtonTone::Primary)
        }
        .render(),
        Button {
            size: Some(Size::Medium),
            ..sample_button("Medium", ButtonTone::Primary)
        }
        .render(),
        Button {
            size: Some(Size::Large),
            ..sample_button("Large", ButtonTone::Primary)
        }
        .render(),
    ])
}

/// Render the calendar specimens.
pub fn calendar() -> Markup {
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
    ])
}

/// Render the card specimens.
pub fn card() -> Markup {
    let action = Button {
        text: "Action".into(),
        tone: Some(ButtonTone::Primary),
        size: Some(Size::Small),
        ..Button::default()
    };
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
    ])
}

/// Render the checkbox specimens.
pub fn checkbox() -> Markup {
    let make = |label: &str, size, checked, disabled, delegated| Checkbox {
        label: label.into(),
        name: None,
        value: None,
        size,
        checked,
        disabled,
        delegated,
        hx: Hx::default(),
    };
    rendered(vec![
        make("Checked checkbox", None, true, false, true).render(),
        make("Unchecked checkbox", None, false, false, true).render(),
        make("Another checked", None, true, false, true).render(),
        make("Small checkbox", Some(Size::Small), true, false, false).render(),
        make("Medium checkbox", Some(Size::Medium), false, false, false).render(),
        make("Large checkbox", Some(Size::Large), true, false, false).render(),
        make("Disabled unchecked", None, false, true, false).render(),
        make("Disabled checked", None, true, true, false).render(),
    ])
}

/// Render the collapsible specimens.
pub fn collapsible() -> Markup {
    let make = |summary: &str, content: &str, preformatted, open, delegated| Collapsible {
        summary: summary.into(),
        content: content.into(),
        preformatted,
        open,
        delegated,
        hx: Hx::default(),
    };
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
    ])
}

/// Render the editable-field specimens.
pub fn editable_field() -> Markup {
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
    ])
}

/// Render the file-input specimens.
pub fn file_input() -> Markup {
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
    ])
}

/// Render the icon-button specimens.
pub fn icon_button() -> Markup {
    let make = |icon, size, shape, label: &str, disabled| IconButton {
        icon,
        size,
        shape,
        label: label.into(),
        disabled,
        hx: Hx::default(),
    };
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
    ])
}

/// Render the input specimens.
pub fn input() -> Markup {
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
    ])
}

/// Render the ip-calendar specimens.
pub fn ip_calendar() -> Markup {
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
    .render()])
}

/// Render the loading-spinner specimens.
pub fn loading_spinner() -> Markup {
    let make = |size, label: Option<&str>, inline, hidden| LoadingSpinner {
        size,
        label: label.map(str::to_owned),
        inline,
        hidden,
        hx: Hx::default(),
    };
    rendered(vec![
        make(Size::Small, Some("Small loading spinner"), false, false).render(),
        make(Size::Medium, Some("Medium loading spinner"), false, false).render(),
        make(Size::Large, Some("Large loading spinner"), false, false).render(),
        make(Size::Small, Some("Loading"), true, false).render(),
        make(Size::Small, None, true, true).render(),
        make(Size::Medium, Some("Loading network data"), false, false).render(),
        make(Size::Large, Some("Connecting to server"), false, false).render(),
    ])
}

/// Render the modal specimens.
pub fn modal() -> Markup {
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
    .render()])
}

/// Render the plus-button specimens.
pub fn plus_button() -> Markup {
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
    ])
}

/// Render the progress-bar specimens.
pub fn progress_bar() -> Markup {
    rendered(vec![
        sample_progress(
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
        sample_progress(
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
        sample_progress(
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
        sample_progress(
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
        sample_progress(
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
        sample_progress(
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
        sample_progress(
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
        sample_progress(
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
        sample_progress(
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
        sample_progress(
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
        sample_progress(
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
        sample_progress(
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
    ])
}

/// Render the row-info-tile specimens.
pub fn row_info_tile() -> Markup {
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
                    sample_badge("PDF", Some(Tone::Primary), None),
                    sample_badge("2.3 MB", Some(Tone::Info), None),
                ],
            },
            hx: Hx::default(),
        }
        .render(),
        RowInfoTile {
            kind: RowKind::Selected {
                checked: true,
                text: "📁 Old".into(),
                badge: sample_badge("Directory", Some(Tone::Success), None),
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
                badge: sample_badge("Error", Some(Tone::Danger), None),
            },
            hx: Hx::default(),
        }
        .render(),
    ])
}

/// Render the select specimens.
pub fn select() -> Markup {
    let make =
        |label: &str, options: &[&str], required, disabled, delegated, error: Option<&str>| {
            Select {
                label: label.into(),
                name: None,
                choices: Vec::new(),
                options: options.iter().map(|value| (*value).into()).collect(),
                required,
                disabled,
                delegated,
                error: error.map(str::to_owned),
                hx: Hx::default(),
            }
        };
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
    ])
}

/// Render the slider specimens.
pub fn slider() -> Markup {
    let make = |min, max, value, step, disabled, dhcp_boundary| Slider {
        min,
        max,
        value,
        step,
        disabled,
        dhcp_boundary,
        hx: Hx::default(),
    };
    rendered(vec![
        make(0, 100, 50, None, false, false).render(),
        make(0, 100, 25, None, false, false).render(),
        make(0, 100, 75, Some(5), false, false).render(),
        make(0, 100, 75, None, true, false).render(),
        make(8, 247, 50, None, false, true).render(),
    ])
}

/// Render the table specimens.
pub fn table() -> Markup {
    let action = |text: &str, modal: &str| Button {
        text: text.into(),
        size: Some(Size::Small),
        button_type: true,
        modal_open: Some(modal.into()),
        interaction_result: Some("modal-open".into()),
        ..Button::default()
    };
    rendered(vec![Table {
        headers: ["Name", "Status", "Version", "Action"]
            .map(str::to_owned)
            .to_vec(),
        rows: vec![
            vec![
                Cell::Text("Data Generator".into()),
                Cell::Badge(sample_badge("active", Some(Tone::Success), None)),
                Cell::Text("1.0.0".into()),
                Cell::Button(action("Inspect", "inspect")),
            ],
            vec![
                Cell::Text("Health Monitor".into()),
                Cell::Badge(sample_badge("active", Some(Tone::Success), None)),
                Cell::Text("1.0.0".into()),
                Cell::Button(action("Run", "run")),
            ],
        ],
        hx: Hx::default(),
    }
    .render()])
}

/// Render the tabs specimens.
pub fn tabs() -> Markup {
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
        sample_tab("Inactive"),
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
    rendered(parts)
}

/// Render the text-box specimens.
pub fn text_box() -> Markup {
    rendered(vec![
            TextBox {
                kind: TextBoxKind::Log, title: Some("System Logs".into()),
                content: "[2025-12-08 18:53:57] [INFO] Starting reinstallation of legacy installer tab: testTab\n[2025-12-08 18:55:28] [INFO] Legacy installer tab 'testTab' reinstalled successfully".into(),
                empty: false, copy_label: Some("Copy".into()), refresh_label: Some("Refresh".into()), delegated: true, hx: Hx::default(),
            }.render(),
            TextBox { kind: TextBoxKind::Code, title: None, content: "function calculateTotal(items) { return items.reduce((sum, item) => sum + (item.price * item.quantity), 0); }".into(), empty: false, copy_label: None, refresh_label: None, delegated: false, hx: Hx::default() }.render(),
            TextBox { kind: TextBoxKind::Terminal, title: None, content: "$ npm install -g serve\n$ serve -s build".into(), empty: false, copy_label: None, refresh_label: None, delegated: false, hx: Hx::default() }.render(),
            TextBox { kind: TextBoxKind::Log, title: None, content: "No log entries yet. Logs will appear here when available.".into(), empty: true, copy_label: None, refresh_label: None, delegated: false, hx: Hx::default() }.render(),
        ])
}

/// Render the time-picker specimens.
pub fn time_picker() -> Markup {
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
    ])
}

/// Render the toggle specimens.
pub fn toggle() -> Markup {
    let make =
        |label: Option<&str>, size, checked, disabled, aria_label: Option<&str>, delegated| {
            Toggle {
                label: label.map(str::to_owned),
                name: None,
                value: None,
                size,
                checked,
                disabled,
                aria_label: aria_label.map(str::to_owned),
                delegated,
                hx: Hx::default(),
            }
        };
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
    ])
}

/// Render the visibility-toggle specimens.
pub fn visibility_toggle() -> Markup {
    let make = |visible, size, label: &str| VisibilityToggle {
        visible,
        size,
        label: label.into(),
        tab_column: false,
        hx: Hx::default(),
    };
    rendered(vec![
        make(true, Some(Size::Small), "Hide small specimen").render(),
        make(false, Some(Size::Medium), "Show medium specimen").render(),
        make(true, Some(Size::Large), "Hide large specimen").render(),
        make(true, None, "Hide visible tab specimen").render(),
        make(false, None, "Show hidden tab specimen").render(),
        make(true, None, "Hide element specimen").render(),
        make(false, None, "Show element specimen").render(),
    ])
}

fn scaffold_example(name: &str, placeholder: &str, fill: &str, state: SlotLifecycle) -> ScaffoldSlot {
    let mut slot = ScaffoldSlot::new("test", name, state).expect("valid slot");
    slot.placeholder = Some(placeholder.into());
    slot.fill = Some(fill.into());
    slot
}

fn sample_slots() -> Vec<ScaffoldSlot> {
    let pending_lines = || vec![SkeletonChild::Stack(vec![SkeletonChild::Line(SkeletonLine::Full), SkeletonChild::Line(SkeletonLine::Medium)])];
    let pending = |message: &str| SlotLifecycle::Pending { children: pending_lines(), message: message.into() };
    let ready = |content| SlotLifecycle::Ready { updated_at: Some(1791993183924), content };
    let fault = |name: &str, error: &str, message: &str, children| SlotLifecycle::Fault { error: error.into(), children, message: message.into(), retry_guest: "test".into(), retry_key: name.into() };
    vec![
        scaffold_example("showcase-skeleton-pending", "summary", "summary", pending("Waiting for the summary…")),
        scaffold_example("showcase-skeleton-ready", "summary", "summary", ready(html! { strong { "Summary ready" } p { "Three regions arrived independently." } })),
        scaffold_example("showcase-skeleton-fault", "summary", "summary", fault("showcase-skeleton-fault", "Read timed out", "Read timed out.", vec![SkeletonChild::Stack(vec![SkeletonChild::Line(SkeletonLine::Short)])])),
        scaffold_example("showcase-slot-pending", "portal details", "details", pending("Loading portal details…")),
        scaffold_example("showcase-slot-ready", "portal details", "details", ready(html! { strong { "Portal details loaded." } p { "Ready content occupies the same slot frame." } })),
        scaffold_example("showcase-slot-fault", "portal details", "details", fault("showcase-slot-fault", "Read failed (HTTP 503)", "Read failed (HTTP 503).", vec![])),
    ]
}

/// Render all three skeleton lifecycle specimens.
pub fn skeleton() -> Markup {
    let slots = sample_slots();
    let labels = ["Pending summary skeleton", "Ready summary frame", "Summary frame with local fault"];
    rendered(labels.into_iter().enumerate().map(|(index, label)| {
        let mut children = Vec::new();
        if index == 0 {
            children.push(SkeletonChild::Row(vec![SkeletonChild::Avatar, SkeletonChild::Stack(vec![SkeletonChild::Line(SkeletonLine::Medium), SkeletonChild::Line(SkeletonLine::Short)])]));
        }
        children.push(SkeletonChild::LifecycleSlot(slots[index].clone()));
        Skeleton::new("test", "1").expect("valid skeleton").with_label(label).with_children(children).render()
    }).collect())
}

/// Render all six scaffold lifecycle specimens.
pub fn slot() -> Markup {
    rendered(sample_slots().into_iter().map(|slot| slot.render()).collect())
}

/// Render visible toast specimens without emitting toast requests.
pub fn toast() -> Markup {
    rendered([
        (ToastKind::Info, "Information notification"),
        (ToastKind::Success, "Settings saved"),
        (ToastKind::Warning, "Review this appliance setting"),
        (ToastKind::Error, "The requested action failed"),
        (ToastKind::Info, "Stack item"),
    ].into_iter().map(|(kind, message)| VisibleToast { kind, message: message.into() }.render()).collect())
}

/// Specimen names in the crown specimen index order.
pub const NAMES: &[&str] = &[
    "badge",
    "breadcrumbs",
    "button",
    "calendar",
    "card",
    "checkbox",
    "collapsible",
    "editable-field",
    "file-input",
    "icon-button",
    "input",
    "ip-calendar",
    "loading-spinner",
    "modal",
    "plus-button",
    "progress-bar",
    "row-info-tile",
    "select",
    "skeleton",
    "slider",
    "slot",
    "table",
    "tabs",
    "text-box",
    "time-picker",
    "toast",
    "toggle",
    "visibility-toggle",
];

/// Render a named specimen, or return `None` for an unknown name.
pub fn section(name: &str) -> Option<Markup> {
    match name {
        "badge" => Some(badge()),
        "breadcrumbs" => Some(breadcrumbs()),
        "button" => Some(button()),
        "calendar" => Some(calendar()),
        "card" => Some(card()),
        "checkbox" => Some(checkbox()),
        "collapsible" => Some(collapsible()),
        "editable-field" => Some(editable_field()),
        "file-input" => Some(file_input()),
        "icon-button" => Some(icon_button()),
        "input" => Some(input()),
        "ip-calendar" => Some(ip_calendar()),
        "loading-spinner" => Some(loading_spinner()),
        "modal" => Some(modal()),
        "plus-button" => Some(plus_button()),
        "progress-bar" => Some(progress_bar()),
        "row-info-tile" => Some(row_info_tile()),
        "select" => Some(select()),
        "skeleton" => Some(skeleton()),
        "slider" => Some(slider()),
        "slot" => Some(slot()),
        "table" => Some(table()),
        "tabs" => Some(tabs()),
        "text-box" => Some(text_box()),
        "time-picker" => Some(time_picker()),
        "toast" => Some(toast()),
        "toggle" => Some(toggle()),
        "visibility-toggle" => Some(visibility_toggle()),
        _ => None,
    }
}

/// Render all named specimens in index order.
pub fn all() -> Markup {
    rendered(NAMES.iter().filter_map(|name| section(name)).collect())
}
