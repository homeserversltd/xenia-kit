//! Category A showcase content composed only from typed xenia-kit builders.
//!
//! The caller owns the enclosing ShowcaseCategory; these functions return only
//! its content.
//! No HTML element tags or class values are authored here.
use crate::{
    html, Markup, Render,
    specimen::{DeviceChoices, DriveChoice, CalendarReadout, TimeReadout, AddressCalendarItem, MonthlyCalendarItem, PortalRemoveSimulator,VisibilityCard,VisibilityTab,VisibilityTabSet,VisibilityNote,VisibilityNoteKind},
    present::{Text, TextKind,Paragraph,CatalogGrid,CatalogKind,ShowcaseArticle,ShowcaseRow,ShowcaseBox},
    layout::{ResolverOption, ResolverSpecimen, ShowcaseGrid, ShowcaseItem, StatusDot},
    ux::{
        Badge, Button, ButtonTone, Calendar, CalendarMode, Card, CardBody, CardState,
        Checkbox, EditableField, EditableSize, Input, IpCalendar, Size,
        Tab, Tabs, TimePicker, Toggle, Tone, VisibilityToggle,
    },
};

fn compose(parts: Vec<Markup>) -> Markup {
    html! { @for part in parts { (part) } }
}

fn item(title: &str, parts: Vec<Markup>) -> Markup {
    ShowcaseItem { title: title.into(), content: compose(parts) }.render()
}

fn grid(items: Vec<Markup>) -> Markup {
    ShowcaseGrid { content: compose(items) }.render()
}

fn button(
    text: &str,
    tone: ButtonTone,
    size: Option<Size>,
    loading: bool,
    disabled: bool,
    left_icon: Option<&str>,
    right_icon: Option<&str>,
) -> Markup {
    Button {
        text: text.into(),
        tone: Some(tone),
        size,
        loading,
        disabled,
        left_icon: left_icon.map(str::to_owned),
        right_icon: right_icon.map(str::to_owned),
        ..Button::default()
    }
    .render()
}

fn toggle(
    label: Option<&str>,
    size: Size,
    checked: bool,
    disabled: bool,
    aria_label: Option<&str>,
) -> Markup {
    Toggle {
        label: label.map(str::to_owned),
        size: Some(size),
        checked,
        disabled,
        aria_label: aria_label.map(str::to_owned),
        delegated: !disabled,
        ..Toggle::default()
    }
    .render()
}

fn badge(text: &str, tone: Option<Tone>, size: Option<Size>) -> Markup {
    Badge { text: text.into(), tone, size, ..Badge::default() }.render()
}

fn checkbox(label: &str, size: Option<Size>, checked: bool, disabled: bool, delegated: bool) -> Markup {
    Checkbox {
        label: label.into(),
        size,
        checked,
        disabled,
        delegated,
        ..Checkbox::default()
    }
    .render()
}

fn tab(label: &str) -> Tab {
    Tab { label: label.into(), ..Tab::default() }
}

fn tabs(tabs: Vec<Tab>) -> Markup {
    Tabs { tabs, ..Tabs::default() }.render()
}

fn category_buttons() -> Markup {
    html!{(grid(vec![
        item("Primary", vec![
            button("Primary Button", ButtonTone::Primary, None, false, false, None, None),
            button("With Icon", ButtonTone::Primary, None, false, false, Some("⌘"), None),
            button("Loading", ButtonTone::Primary, None, true, false, None, None),
            button("Disabled", ButtonTone::Primary, None, false, true, None, None),
        ]),
        item("Secondary", vec![
            button("Secondary Button", ButtonTone::Secondary, None, false, false, None, None),
            button("Icon Right", ButtonTone::Secondary, None, false, false, None, Some("👤")),
            button("Disabled", ButtonTone::Secondary, None, false, true, None, None),
        ]),
        item("Danger", vec![
            button("Danger Button", ButtonTone::Danger, None, false, false, None, None),
            button("Disabled", ButtonTone::Danger, None, false, true, None, None),
        ]),
        item("Warning", vec![
            button("Warning Button", ButtonTone::Warning, None, false, false, None, None),
            button("Disabled", ButtonTone::Warning, None, false, true, None, None),
        ]),
        item("Success", vec![
            button("Success Button", ButtonTone::Success, None, false, false, None, None),
            button("Disabled", ButtonTone::Success, None, false, true, None, None),
        ]),
    ]))(Text{kind:TextKind::Heading3,content:html!{"Button Sizes"}})
    (ShowcaseRow{modal_controls:false,content:html!{
        (button("Small", ButtonTone::Primary, Some(Size::Small), false, false, None, None))
        (button("Medium", ButtonTone::Primary, Some(Size::Medium), false, false, None, None))
        (button("Large", ButtonTone::Primary, Some(Size::Large), false, false, None, None))
    }})}
}

fn category_toggles() -> Markup {
    grid(vec![
        item("Small", vec![toggle(Some("Small Toggle"), Size::Small, false, false, None)]),
        item("Medium (Default)", vec![toggle(Some("Medium Toggle"), Size::Medium, true, false, None)]),
        item("Large", vec![toggle(Some("Large Toggle"), Size::Large, false, false, None)]),
        item("Without Label", vec![toggle(None, Size::Medium, false, false, Some("Toggle without label"))]),
        item("Disabled", vec![
            toggle(Some("Disabled Off"), Size::Medium, false, true, None),
            toggle(Some("Disabled On"), Size::Medium, true, true, None),
        ]),
    ])
}

fn category_tabs() -> Markup {
    let article=|title:&str, parts:Vec<Markup>| ShowcaseArticle{title:title.into(),content:html!{@for p in parts {(p)}}}.render();
    let intro=Text{kind:TextKind::Paragraph,content:html!{"The " (Text{kind:TextKind::Code,content:html!{"ui-tab"}}) " component is the one primitive behind the document tab bar, the pane-section tabs above, and the category chips below — depth, icon, visibility column, and star column are all part of the same component, defined once in " (Text{kind:TextKind::Code,content:html!{"_tabs.css"}}) "."}};
    let rows=grid(vec![
        article("Sizes", vec![ShowcaseRow{modal_controls:false,content:html!{
            (tabs(vec![Tab { label: "Small".into(), active: true, size: Some(Size::Small), ..Tab::default() }]))
            (tabs(vec![Tab { label: "Medium".into(), active: true, size: Some(Size::Medium), ..Tab::default() }]))
            (tabs(vec![Tab { label: "Large".into(), active: true, size: Some(Size::Large), ..Tab::default() }]))
        }}.render()]),
        article("States", vec![tabs(vec![
            Tab { label: "Active".into(), active: true, ..Tab::default() },
            tab("Inactive"),
            Tab { label: "Disabled".into(), disabled: true, ..Tab::default() },
            Tab { label: "Hidden".into(), hidden: true, ..Tab::default() },
            Tab { label: "Admin-only".into(), admin_only: true, ..Tab::default() },
        ])]),
        article("With icon", vec![tabs(vec![
            Tab { label: "Home".into(), icon: Some("⌘".into()), active: true, ..Tab::default() },
            Tab { label: "Profile".into(), icon: Some("👤".into()), ..Tab::default() },
            Tab { label: "Settings".into(), icon: Some("⚙".into()), ..Tab::default() },
        ])]),
        article("Visibility column", vec![Paragraph{text:"The same eye toggle every live tab bar carries — click to compare.".into()}.render(),tabs(vec![
            Tab {
                label: "Stats".into(), active: true,
                visibility: Some(VisibilityToggle { visible: true, label: "Hide Stats tab".into(), tab_column: true, ..VisibilityToggle::default() }),
                ..Tab::default()
            },
            Tab {
                label: "DHCP".into(),
                visibility: Some(VisibilityToggle { visible: false, label: "Show DHCP tab".into(), tab_column: true, ..VisibilityToggle::default() }),
                ..Tab::default()
            },
        ])]),
        article("Star column", vec![tabs(vec![
            Tab { label: "backBlaze".into(), active: true, starred: Some(true), star_label: Some("backBlaze tab is starred".into()), ..Tab::default() },
            Tab { label: "Upload".into(), starred: Some(false), star_label: Some("Star Upload tab".into()), ..Tab::default() },
        ])]),
        article("Eye and star together", vec![Paragraph{text:"When both columns are present the label centers between them.".into()}.render(),tabs(vec![Tab {
            label: "testing".into(), active: true,
            visibility: Some(VisibilityToggle { visible: true, label: "Hide testing tab".into(), tab_column: true, ..VisibilityToggle::default() }),
            starred: Some(true), star_label: Some("testing tab is starred".into()),
            ..Tab::default()
        }])]),
    ]);
    let nested=ShowcaseBox{content:html!{
        (tabs(vec![Tab { label: "Depth 1 — document tab".into(), active: true, depth: Some(1), ..Tab::default() }, Tab { label: "Sibling".into(), depth: Some(1), ..Tab::default() }]))
        (tabs(vec![Tab { label: "Depth 2 — pane section".into(), active: true, depth: Some(2), ..Tab::default() }, Tab { label: "Sibling".into(), depth: Some(2), ..Tab::default() }]))
        (tabs(vec![Tab { label: "Depth 3 — category chip".into(), active: true, depth: Some(3), ..Tab::default() }, Tab { label: "Sibling".into(), depth: Some(3), ..Tab::default() }]))
    }};
    let overflow=ShowcaseBox{content:tabs([
        "Upload", "Stats", "backBlaze", "DHCP", "Firewall", "Wake on LAN", "Linker", "Test", "testing",
    ].into_iter().enumerate().map(|(index, label)| Tab { label: label.into(), active: index == 0, ..Tab::default() }).collect())};
    html!{(intro)(rows)(Text{kind:TextKind::Heading4,content:html!{"Nested depth"}})
        (Paragraph{text:"The document tab bar, in-pane sections, and category chips are the same component at three depths — only the left-edge indicator weight changes.".into()})(nested)
        (Text{kind:TextKind::Heading4,content:html!{"Overflow"}})(Paragraph{text:"The group wraps instead of clipping once a row runs out of width.".into()})(overflow)}
}

fn category_inputs() -> Markup {
    grid(vec![
        item("Text Input", vec![Input { label: Some("Username".into()), kind: Some("text".into()), placeholder: Some("Enter username".into()), delegated: true, ..Input::default() }.render()]),
        item("Input with Error", vec![Input { label: Some("Email".into()), kind: Some("text".into()), value: Some("ab".into()), error: Some("Input must be at least 3 characters".into()), delegated: true, ..Input::default() }.render()]),
        item("Password Input", vec![Input { kind: Some("password".into()), placeholder: Some("Enter password".into()), ..Input::default() }.render()]),
        item("Required Field", vec![Input { required: true, placeholder: Some("This field is required".into()), ..Input::default() }.render()]),
        item("Disabled Input", vec![Input { value: Some("Cannot edit".into()), disabled: true, ..Input::default() }.render()]),
        item("Display Variant", vec![Input { value: Some("homeserver".into()), display: true, readonly: true, ..Input::default() }.render()]),
        item("Input Sizes", vec![
            Input { size: Some(Size::Small), placeholder: Some("Small input".into()), ..Input::default() }.render(),
            Input { size: Some(Size::Medium), placeholder: Some("Medium input".into()), ..Input::default() }.render(),
            Input { size: Some(Size::Large), placeholder: Some("Large input".into()), ..Input::default() }.render(),
        ]),
    ])
}

fn category_cards() -> Markup {
    let action = Button { text: "Action".into(), tone: Some(ButtonTone::Primary), size: Some(Size::Small), ..Button::default() };
    grid(vec![
        item("Default Card", vec![Card { body: CardBody::Paragraph("This is a default card with some content.".into()), ..Card::default() }.render()]),
        item("Card with Header", vec![Card { body: CardBody::Paragraph("Card body content goes here.".into()), header: Some("Card Header".into()), ..Card::default() }.render()]),
        item("Card with Footer", vec![Card { body: CardBody::Text("Card with footer actions.".into()), footer: vec![action], body_wrapper: true, ..Card::default() }.render()]),
        item("Clickable Card", vec![Card { body: CardBody::Text("Click this card to interact.".into()), state: Some(CardState::Clickable), ..Card::default() }.render()]),
        item("Active Card", vec![Card { body: CardBody::Text("This card indicates an active state.".into()), state: Some(CardState::Active), ..Card::default() }.render()]),
        item("Error Card", vec![Card { body: CardBody::Text("This card indicates an error state.".into()), state: Some(CardState::Error), ..Card::default() }.render()]),
        item("Portal remove simulator",vec![Text{kind:TextKind::MutedParagraph,content:html!{"Simulator only — this control never changes a portal."}}.render(),PortalRemoveSimulator{name:"Simulator portal".into(),description:"Quiet remove-control specimen".into(),visible:true,visibility_label:"Hide simulator portal".into(),delete_label:"Delete simulator portal".into()}.render()]),
    ])
}

fn category_badges() -> Markup {
    grid(vec![
        item("Badge Variants", vec![ShowcaseRow{modal_controls:false,content:html!{
            (badge("Primary", Some(Tone::Primary), None)) (badge("Secondary", Some(Tone::Secondary), None))
            (badge("Success", Some(Tone::Success), None)) (badge("Warning", Some(Tone::Warning), None))
            (badge("Danger", Some(Tone::Danger), None)) (badge("Info", Some(Tone::Info), None))
        }}.render()]),
        item("Badge Sizes", vec![badge("Small", None, Some(Size::Small)), badge("Medium", None, Some(Size::Medium)), badge("Large", None, Some(Size::Large))]),
        item("Badge Usage Examples", vec![
            Text{kind:TextKind::Div,content:html! { "Status: " (badge("Online", Some(Tone::Success), None)) }}.render(),
            Text{kind:TextKind::Div,content:html! { "Count: " (badge("42", Some(Tone::Primary), None)) }}.render(),
            Text{kind:TextKind::Div,content:html! { "Alert: " (badge("Critical", Some(Tone::Danger), None)) }}.render(),
        ]),
    ])
}

fn category_checkboxes() -> Markup {
    grid(vec![
        item("Basic Checkboxes", vec![
            checkbox("Checked checkbox", None, true, false, true),
            checkbox("Unchecked checkbox", None, false, false, true),
            checkbox("Another checked", None, true, false, true),
        ]),
        item("Checkbox Sizes", vec![
            checkbox("Small checkbox", Some(Size::Small), true, false, false),
            checkbox("Medium checkbox", Some(Size::Medium), false, false, false),
            checkbox("Large checkbox", Some(Size::Large), true, false, false),
        ]),
        item("Disabled Checkboxes", vec![
            checkbox("Disabled unchecked", None, false, true, false),
            checkbox("Disabled checked", None, true, true, false),
        ]),
        item("Stats Tab Style (Grouped)",vec![DeviceChoices{device:"nas".into(),choices:vec![DriveChoice{label:"Read".into(),checked:true},DriveChoice{label:"Write".into(),checked:false}]}.render()]),
    ])
}

fn category_utilities() -> Markup {
    grid(vec![
        item("Plus Button", vec![
            crate::ux::PlusButton { size: Some(Size::Small), ..Default::default() }.render(),
            crate::ux::PlusButton { size: Some(Size::Medium), ..Default::default() }.render(),
            crate::ux::PlusButton { size: Some(Size::Large), ..Default::default() }.render(),
            crate::ux::PlusButton { secondary: true, ..Default::default() }.render(),
            crate::ux::PlusButton { disabled: true, ..Default::default() }.render(),
        ]),
        item("Editable Field", vec![
            EditableField { value: "Click to edit this text".into(), size: Some(EditableSize::Medium), ..EditableField::default() }.render(),
            EditableField { value: "Empty field - click to edit".into(), size: Some(EditableSize::Small), placeholder: true, ..EditableField::default() }.render(),
            EditableField { value: "Disabled field".into(), disabled: true, ..EditableField::default() }.render(),
        ]),
        item("Status Dot", vec![
            Text{kind:TextKind::Div,content:html! { "Status: " (StatusDot::Up) " Online" }}.render(),
            Text{kind:TextKind::Div,content:html! { "Status: " (StatusDot::Down) " Offline" }}.render(),
            Text{kind:TextKind::Div,content:html! { "Status: " (StatusDot::Unknown) " Unknown" }}.render(),
        ]),
    ])
}

fn category_visibility_system() -> Markup {
    let size=ShowcaseArticle{title:"Toggle sizes".into(),content:ShowcaseRow{modal_controls:false,content:html!{
        (VisibilityToggle { visible: true, size: Some(Size::Small), label: "Hide small specimen".into(), ..VisibilityToggle::default() })
        (VisibilityToggle { visible: false, size: Some(Size::Medium), label: "Show medium specimen".into(), ..VisibilityToggle::default() })
        (VisibilityToggle { visible: true, size: Some(Size::Large), label: "Hide large specimen".into(), ..VisibilityToggle::default() })
    }}.render()};
    let tab=ShowcaseArticle{title:"Tab states".into(),content:VisibilityTabSet{tabs:vec![
        VisibilityTab{label:"Visible tab".into(),visible:true,active:true,control_label:"Hide visible tab specimen".into()},
        VisibilityTab{label:"Hidden tab".into(),visible:false,active:false,control_label:"Show hidden tab specimen".into()},
    ],admin_label:"Admin-only tab".into()}.render()};
    let notes=html!{
        (VisibilityNote{kind:VisibilityNoteKind::ElementsChanged,title:"ElementsChanged Pull".into(),content:html!{"One data-free " (Text{kind:TextKind::Code,content:html!{"elements.changed"}}) " poke asks each held pane to refresh only its own session-projected element fragment; the active pane remains unchanged."}})
        (VisibilityNote{kind:VisibilityNoteKind::LiveMorph,title:"Live-pane DOM morph".into(),content:html!{"Live panes are morphed, never " (Text{kind:TextKind::Code,content:html!{"innerHTML"}}) "-replaced. Matching element facts leave every DOM node untouched; changed facts reconcile by id, then position. VIS-004 records the flicker defect: replacing a live stats pane tears down its chart frame on every tick."}})
    };
    html!{(Paragraph{text:"Click each eye to compare the visible and hidden presentation.".into()})(CatalogGrid{kind:CatalogKind::Visibility,content:html!{
        (size)(tab)
        (VisibilityCard{title:"Element visible".into(),visible:true,control_label:"Hide element specimen".into(),state_label:"Visible".into()})
        (VisibilityCard{title:"Element dimmed hidden".into(),visible:false,control_label:"Show element specimen".into(),state_label:"Dimmed hidden".into()})
        (notes)
    }})}
}

fn category_calendar_time() -> Markup {
    let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]
        .into_iter().map(str::to_owned).collect();
    let month_days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
        .into_iter().map(str::to_owned).collect();
    grid(vec![
        item("Calendar - Weekly Mode", vec![Calendar { mode: CalendarMode::Weekdays(weekdays), hx: Default::default() }.render(),CalendarReadout{value:String::new()}.render()]),
        MonthlyCalendarItem{title:"Calendar - Monthly Mode".into(),content:Calendar {
            mode: CalendarMode::Monthly {
                previous: "Previous".into(), next: "Next".into(),
                previous_label: "Previous month".into(), next_label: "Next month".into(),
                month: String::new(), weekdays: month_days, output: String::new(),
            },
            hx: Default::default(),
        }.render()}.render(),
        item("Time Picker", vec![
            TimePicker { value: "03:00".into(), delegated: true, ..TimePicker::default() }.render(),
            TimeReadout{before:"Selected: ".into(),value:"03:00".into(),after:" (24-hour format)".into()}.render(),
        ]),
        item("Disabled States", vec![Calendar { mode: CalendarMode::Disabled("Monday".into()), hx: Default::default() }.render(),TimePicker{value:"09:00".into(),disabled:true,..TimePicker::default()}.render()]),
    ])
}

fn category_lan_ip_calendar() -> Markup {
    AddressCalendarItem{title:"LAN IP octet calendar".into(),description:html!{(Text{kind:TextKind::Code,content:html!{"192.168.123."}}) " Select an available reservation octet. Taken addresses are named and unavailable."},content:IpCalendar {
        prefix: "192.168.123.".into(), range: "100 - 131".into(),
        previous: "Previous".into(), next: "Next".into(),
        previous_label: "Previous address range".into(), next_label: "Next address range".into(),
        grid_label: "LAN reservation addresses".into(),
        readout: "Selected address: 192.168.123.100".into(),
        ..IpCalendar::default()
    }.render()}.render()
}

fn category_upstream_resolvers() -> Markup {
    ResolverSpecimen {
        title: "Upstream resolver checkbox set".into(),
        description: "Choose one or more public upstreams. This specimen composes addresses and encrypted-DNS state without changing resolver settings.".into(),
        controls: crate::layout::UpstreamResolvers {
            providers: vec![
                ResolverOption { value: "quad9".into(), label: "Quad9".into(), addresses: "9.9.9.9, 149.112.112.112".into() },
                ResolverOption { value: "cloudflare".into(), label: "Cloudflare".into(), addresses: "1.1.1.1, 1.0.0.1".into() },
                ResolverOption { value: "google".into(), label: "Google".into(), addresses: "8.8.8.8, 8.8.4.4".into() },
            ],
            custom_label: "External addresses (comma-separated)".into(),
            custom_placeholder: "208.67.222.222, 208.67.220.220".into(),
            encrypted_label: "Use encrypted DNS (DoT)".into(),
            readout: "Upstreams: [] - DoT: off".into(),
        },
    }.render()
}

/// Return category content only; the parent supplies each enclosing category.
pub fn section(id: &str) -> Option<Markup> {
    match id {
        "buttons" => Some(category_buttons()),
        "toggles" => Some(category_toggles()),
        "tabs" => Some(category_tabs()),
        "inputs" => Some(category_inputs()),
        "cards" => Some(category_cards()),
        "badges" => Some(category_badges()),
        "checkboxes" => Some(category_checkboxes()),
        "utilities" => Some(category_utilities()),
        "visibility-system" => Some(category_visibility_system()),
        "calendar-time" => Some(category_calendar_time()),
        "lan-ip-calendar" => Some(category_lan_ip_calendar()),
        "upstream-resolvers" => Some(category_upstream_resolvers()),
        _ => None,
    }
}