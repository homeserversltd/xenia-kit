//! Typed library markup. Text is escaped by maud; no caller-supplied classes.
use maud::{html, Markup, Render};

#[derive(Clone, Copy, Debug)]
pub enum Swap {
    InnerHtml,
    OuterHtml,
    BeforeBegin,
    AfterBegin,
    BeforeEnd,
    AfterEnd,
    Delete,
    None,
}
impl Swap {
    fn name(self) -> &'static str {
        match self {
            Self::InnerHtml => "innerHTML",
            Self::OuterHtml => "outerHTML",
            Self::BeforeBegin => "beforebegin",
            Self::AfterBegin => "afterbegin",
            Self::BeforeEnd => "beforeend",
            Self::AfterEnd => "afterend",
            Self::Delete => "delete",
            Self::None => "none",
        }
    }
}
#[derive(Clone, Default, Debug)]
pub struct Hx {
    pub get: Option<String>,
    pub post: Option<String>,
    pub target: Option<String>,
    pub swap: Option<Swap>,
    pub trigger: Option<String>,
}
impl Hx {
    fn apply(&self, mut markup: Markup) -> Markup {
        let mut attrs = String::new();
        for (key, value) in [
            ("hx-get", self.get.as_deref()),
            ("hx-post", self.post.as_deref()),
            ("hx-target", self.target.as_deref()),
            ("hx-swap", self.swap.map(Swap::name)),
            ("hx-trigger", self.trigger.as_deref()),
        ] {
            if let Some(value) = value {
                attrs.push_str(&format!(" {key}=\"{}\"", html! {(value)}.0));
            }
        }
        if !attrs.is_empty() {
            if let Some(at) = markup.0.find('>') {
                markup.0.insert_str(at, &attrs);
            }
        }
        markup
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Size {
    Small,
    Medium,
    Large,
}
impl Size {
    fn name(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Tone {
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Info,
}
impl Tone {
    fn name(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
            Self::Info => "info",
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum ButtonTone {
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
}
impl ButtonTone {
    fn name(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum EditableSize {
    Small,
    Medium,
}
impl EditableSize {
    fn name(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
        }
    }
}
fn classes(base: &str, modifiers: &[Option<&str>]) -> String {
    let mut c = base.to_string();
    for m in modifiers.iter().flatten() {
        c.push_str(&format!(" {base}--{m}"));
    }
    c
}
macro_rules! renderer {
    ($t:ty,$s:ident,$body:block) => {
        impl Render for $t {
            fn render(&self) -> Markup {
                let $s = self;
                let body: Markup = $body;
                $s.hx.apply(body)
            }
        }
    };
}

#[derive(Clone, Default)]
pub struct Badge {
    pub text: String,
    pub tone: Option<Tone>,
    pub size: Option<Size>,
    pub hx: Hx,
}
impl Badge {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }
}
renderer!(Badge, s, {
    html! {span class=(classes("ui-badge",&[s.tone.map(Tone::name),s.size.map(Size::name)])){(&s.text)}}
});

#[derive(Clone, Default)]
pub struct Button {
    pub text: String,
    pub tone: Option<ButtonTone>,
    pub size: Option<Size>,
    pub loading: bool,
    pub disabled: bool,
    pub left_icon: Option<String>,
    pub right_icon: Option<String>,
    pub button_type: bool,
    pub modal_open: Option<String>,
    pub interaction_result: Option<String>,
    pub hx: Hx,
}
impl Button {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }
}
renderer!(Button, s, {
    html! {button type=[s.button_type.then_some("button")] class=(classes("ui-button",&[s.tone.map(ButtonTone::name),s.loading.then_some("loading"),s.size.map(Size::name)])) aria-busy=[s.loading.then_some("true")] disabled[s.disabled] data-ux-modal-open=[s.modal_open.as_deref()] data-interaction-result=[s.interaction_result.as_deref()] {
        @if let Some(icon)=&s.left_icon {span class="ui-button__icon ui-button__icon--left" {(icon)}}
        (&s.text)
        @if let Some(icon)=&s.right_icon {" " span class="ui-button__icon ui-button__icon--right" {(icon)}}
    }}
});

#[derive(Clone)]
pub enum CardBody {
    Text(String),
    Paragraph(String),
}
impl Default for CardBody {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
impl Render for CardBody {
    fn render(&self) -> Markup {
        match self {
            Self::Text(t) => html! {(t)},
            Self::Paragraph(t) => html! {p {(t)}},
        }
    }
}
#[derive(Clone, Copy)]
pub enum CardState {
    Clickable,
    Active,
    Error,
}
#[derive(Clone, Default)]
pub struct Card {
    pub body: CardBody,
    pub header: Option<String>,
    pub footer: Vec<Button>,
    pub body_wrapper: bool,
    pub state: Option<CardState>,
    pub hx: Hx,
}
renderer!(Card, s, {
    let class = classes(
        "ui-card",
        &[s.state.map(|v| match v {
            CardState::Clickable => "clickable",
            CardState::Active => "active",
            CardState::Error => "error",
        })],
    );
    if matches!(s.state, Some(CardState::Clickable)) {
        html! {button class=(class){(&s.body)}}
    } else {
        html! {div class=(class){
            @if let Some(header)=&s.header {div class="ui-card__header"{(header)}}
            @if s.body_wrapper || s.header.is_some() || !s.footer.is_empty(){div class="ui-card__body"{(&s.body)}}@else{(&s.body)}
            @if !s.footer.is_empty(){div class="ui-card__footer"{@for action in &s.footer{(action)}}}
        }}
    }
});
#[derive(Clone, Default)]
pub struct Toggle {
    pub label: Option<String>,
    pub size: Option<Size>,
    pub checked: bool,
    pub disabled: bool,
    pub aria_label: Option<String>,
    pub delegated: bool,
    pub hx: Hx,
}
renderer!(Toggle, s, {
    html! {label class=(classes("ui-toggle",&[s.size.map(Size::name),s.disabled.then_some("disabled")])) {span class="ui-toggle__switch" {input class="ui-toggle__input" type="checkbox" checked[s.checked] disabled[s.disabled] aria-label=[s.aria_label.as_deref()] data-ui-toggle[s.delegated];span class="ui-toggle__slider"{}} @if let Some(label)=&s.label {span class="ui-toggle__label"{(label)}}}}
});
#[derive(Clone)]
pub enum Cell {
    Text(String),
    Badge(Badge),
    Button(Button),
}
impl Render for Cell {
    fn render(&self) -> Markup {
        match self {
            Self::Text(t) => html! {(t)},
            Self::Badge(b) => b.render(),
            Self::Button(b) => b.render(),
        }
    }
}
#[derive(Clone, Default)]
pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<Cell>>,
    pub hx: Hx,
}
renderer!(Table, s, {
    html! {div class="ui-table-container"{table class="ui-table"{thead{tr{@for h in &s.headers{th{(h)}}}}tbody{@for row in &s.rows{tr{@for cell in row{td{(cell)}}}}}}}}
});

#[derive(Clone, Default)]
pub struct Checkbox {
    pub label: String,
    pub size: Option<Size>,
    pub checked: bool,
    pub disabled: bool,
    pub delegated: bool,
    pub hx: Hx,
}
renderer!(Checkbox, s, {
    html! {label class=(classes("ui-checkbox",&[s.size.map(Size::name)])){input type="checkbox" checked[s.checked] disabled[s.disabled] data-ui-checkbox[s.delegated];(&s.label)}}
});
#[derive(Clone, Default)]
pub struct Collapsible {
    pub summary: String,
    pub content: String,
    pub preformatted: bool,
    pub open: bool,
    pub delegated: bool,
    pub hx: Hx,
}
renderer!(Collapsible, s, {
    html! {details class="collapsible" open[s.open] data-ui-collapsible[s.delegated]{summary{(&s.summary)}@if s.preformatted{pre{(&s.content)}}@else{p{(&s.content)}}}}
});
#[derive(Clone, Default)]
pub struct EditableField {
    pub value: String,
    pub size: Option<EditableSize>,
    pub placeholder: bool,
    pub disabled: bool,
    pub hx: Hx,
}
renderer!(EditableField, s, {
    html! {span class=(classes("ui-editable-field",&[s.size.map(EditableSize::name),s.disabled.then_some("disabled")])) contenteditable=[(!s.disabled).then_some("true")]{@if s.disabled{(&s.value)}@else{span class=(if s.placeholder{"ui-editable-field__value ui-editable-field__placeholder"}else{"ui-editable-field__value"}){(&s.value)}}}}
});
#[derive(Clone, Default)]
pub struct Input {
    pub label: Option<String>,
    pub value: Option<String>,
    pub placeholder: Option<String>,
    pub kind: Option<String>,
    pub size: Option<Size>,
    pub error: Option<String>,
    pub display: bool,
    pub required: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub delegated: bool,
    pub hx: Hx,
}
renderer!(Input, s, {
    let field = html! {input class=(classes("ui-input",&[s.error.as_ref().map(|_|"error"),s.display.then_some("display"),s.size.map(Size::name)])) type=[s.kind.as_deref()] required[s.required] placeholder=[s.placeholder.as_deref()] value=[s.value.as_deref()] disabled[s.disabled] readonly[s.readonly] data-ui-input[s.delegated];};
    if let Some(label) = &s.label {
        html! {label class="ui-input-label"{(label)(field)@if let Some(error)=&s.error{span class="ui-input__error"{(error)}}}}
    } else {
        field
    }
});
#[derive(Clone, Default)]
pub struct Select {
    pub label: String,
    pub options: Vec<String>,
    pub required: bool,
    pub disabled: bool,
    pub delegated: bool,
    pub error: Option<String>,
    pub hx: Hx,
}
renderer!(Select, s, {
    html! {label class=(if s.error.is_some(){"ui-select-label ui-select--error"}else{"ui-select-label"}){(&s.label)select class="ui-select" data-ui-select[s.delegated] required[s.required] disabled[s.disabled]{@for option in &s.options{option{(option)}}}@if let Some(error)=&s.error{span class="ui-input__error"{(error)}}}}
});
#[derive(Clone, Default)]
pub struct PlusButton {
    pub size: Option<Size>,
    pub secondary: bool,
    pub disabled: bool,
    pub hx: Hx,
}
renderer!(PlusButton, s, {
    html! {button class=(classes("ui-plus-button",&[s.size.map(Size::name),s.secondary.then_some("secondary")])) disabled[s.disabled]{"+"}}
});
#[derive(Clone, Copy)]
pub enum Icon {
    Refresh,
    Sync,
    Settings,
}
impl Icon {
    fn class(self) -> &'static str {
        match self {
            Self::Refresh => "fas fa-rotate",
            Self::Sync => "fas fa-sync",
            Self::Settings => "fas fa-cog",
        }
    }
}
#[derive(Clone, Copy)]
pub enum Shape {
    Square,
    Circle,
}
#[derive(Clone)]
pub struct IconButton {
    pub icon: Icon,
    pub size: Size,
    pub shape: Shape,
    pub label: String,
    pub disabled: bool,
    pub hx: Hx,
}
renderer!(IconButton, s, {
    html! {button type="button" class=(classes("ui-icon-button",&[Some(s.size.name()),Some(match s.shape{Shape::Square=>"square",Shape::Circle=>"circle"})])) disabled[s.disabled] aria-label=(&s.label){i class=(s.icon.class()) aria-hidden="true"{}}}
});
#[derive(Clone, Default)]
pub struct VisibilityToggle {
    pub visible: bool,
    pub size: Option<Size>,
    pub label: String,
    pub tab_column: bool,
    pub hx: Hx,
}
renderer!(VisibilityToggle, s, {
    let mut class = classes(
        "ui-visibility-toggle",
        &[
            Some(if s.visible { "visible" } else { "hidden" }),
            s.size.map(Size::name),
        ],
    );
    if s.tab_column {
        class = format!("ui-tab__visibility-toggle {class}");
    }
    html! {button type="button" class=(class) data-ui-visibility-toggle data-visible=(s.visible.to_string()) aria-pressed=(s.visible.to_string()) aria-label=(&s.label){i class=(if s.visible{"fas fa-eye"}else{"fas fa-eye-slash"}) aria-hidden="true"{}}}
});
#[derive(Clone)]
pub struct LoadingSpinner {
    pub size: Size,
    pub label: Option<String>,
    pub inline: bool,
    pub hidden: bool,
    pub hx: Hx,
}
renderer!(LoadingSpinner, s, {
    let class = format!("loading-spinner {}", s.size.name());
    if s.inline {
        html! {span class=(class) role=[(!s.hidden).then_some("progressbar")] aria-label=[s.label.as_deref()] aria-hidden=[s.hidden.then_some("true")] {}}
    } else {
        html! {div class=(class) role=[(!s.hidden).then_some("progressbar")] aria-label=[s.label.as_deref()] aria-hidden=[s.hidden.then_some("true")] {}}
    }
});
#[derive(Clone, Default)]
pub struct TimePicker {
    pub value: String,
    pub disabled: bool,
    pub delegated: bool,
    pub hx: Hx,
}
renderer!(TimePicker, s, {
    html! {input class="time-picker" type=[(!s.disabled).then_some("time")] value=(&s.value) disabled[s.disabled] data-ui-time-picker[s.delegated];}
});
#[derive(Clone)]
pub struct Breadcrumb {
    pub text: String,
    pub path: String,
    pub current: bool,
}
#[derive(Clone, Default)]
pub struct Breadcrumbs {
    pub label: String,
    pub items: Vec<Breadcrumb>,
    pub delegated: bool,
    pub hx: Hx,
}
renderer!(Breadcrumbs, s, {
    html! {nav class="ui-breadcrumbs" data-test-upload-breadcrumbs[s.delegated] aria-label=(&s.label){@for (i,item) in s.items.iter().enumerate(){@if i>0{span class="ui-breadcrumbs__separator"{" / "}}button type="button" class=(if item.current{"ui-breadcrumbs__item ui-breadcrumbs__item--current"}else{"ui-breadcrumbs__item"}) data-test-breadcrumb-path=(&item.path) aria-current=[item.current.then_some("page")]{(&item.text)}}}}
});
#[derive(Clone)]
pub enum CalendarMode {
    Weekdays(Vec<String>),
    Monthly {
        previous: String,
        next: String,
        previous_label: String,
        next_label: String,
        month: String,
        weekdays: Vec<String>,
        output: String,
    },
    Disabled(String),
}
#[derive(Clone)]
pub struct Calendar {
    pub mode: CalendarMode,
    pub hx: Hx,
}
renderer!(Calendar, s, {
    match &s.mode {
        CalendarMode::Weekdays(days) => {
            html! {select class="calendar-picker" data-ui-calendar{@for day in days{option{(day)}}}}
        }
        CalendarMode::Disabled(value) => {
            html! {input class="calendar-picker" value=(value) disabled;}
        }
        CalendarMode::Monthly {
            previous,
            next,
            previous_label,
            next_label,
            month,
            weekdays,
            output,
        } => {
            html! {div class="ui-calendar-monthly"{div class="ui-calendar-monthly__chrome"{button type="button" class="ui-button ui-button--secondary ui-button--small" data-ui-calendar-monthly-prev aria-label=(previous_label){(previous)}strong data-ui-calendar-monthly-month{(month)}button type="button" class="ui-button ui-button--secondary ui-button--small" data-ui-calendar-monthly-next aria-label=(next_label){(next)}}div class="ui-calendar-monthly__weekdays"{@for day in weekdays{span{(day)}}}div class="ui-calendar-monthly__grid" role="grid" data-ui-calendar-monthly-grid{}p data-ui-calendar-output data-ui-calendar-monthly-output aria-live="polite"{(output)}}}
        }
    }
});
#[derive(Clone, Default)]
pub struct IpCalendar {
    pub prefix: String,
    pub range: String,
    pub previous: String,
    pub next: String,
    pub previous_label: String,
    pub next_label: String,
    pub grid_label: String,
    pub readout: String,
    pub hx: Hx,
}
renderer!(IpCalendar, s, {
    html! {div class="ui-lan-ip-calendar"{div class="ui-lan-ip-calendar__chrome"{span class="ui-lan-ip-calendar__prefix"{(&s.prefix)}div class="ui-lan-ip-calendar__pager"{button type="button" class="ui-button ui-button--secondary ui-button--small" data-lan-ip-prev aria-label=(&s.previous_label){(&s.previous)}strong data-lan-ip-range{(&s.range)}button type="button" class="ui-button ui-button--secondary ui-button--small" data-lan-ip-next aria-label=(&s.next_label){(&s.next)}}}div class="ui-lan-ip-calendar__grid" role="grid" aria-label=(&s.grid_label) data-lan-ip-grid{}p class="ui-lan-ip-calendar__readout" data-lan-ip-readout aria-live="polite"{(&s.readout)}}}
});
#[derive(Clone)]
pub struct Slider {
    pub min: i64,
    pub max: i64,
    pub value: i64,
    pub step: Option<u64>,
    pub disabled: bool,
    pub dhcp_boundary: bool,
    pub hx: Hx,
}
renderer!(Slider, s, {
    let percent = if s.max <= s.min {
        0.0
    } else {
        ((s.value as f64 - s.min as f64) / (s.max as f64 - s.min as f64) * 100.0).clamp(0.0, 100.0)
    };
    let pct = format!("{percent:.4}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string();
    html! {div class="ui-slider__container"{div class="ui-slider__track"{}div class="ui-slider__fill" data-slider-fill style=(format!("width:{pct}%")){}input class="ui-slider__input" type="range" min=(s.min) max=(s.max) step=[s.step] value=(s.value) data-ui-slider disabled[s.disabled] data-dhcp-example-boundary[s.dhcp_boundary];}}
});
#[derive(Clone, Default)]
pub struct FileInput {
    pub label: String,
    pub button: String,
    pub display: String,
    pub input_label: String,
    pub display_label: String,
    pub multiple: bool,
    pub disabled: bool,
    pub error: Option<String>,
    pub delegated: bool,
    pub hx: Hx,
}
renderer!(FileInput, s, {
    html! {label class="ui-file-input" data-test-file-input[s.delegated]{span class="ui-file-input__label"{(&s.label)}span class="ui-file-input__controls"{
    @if !s.disabled && s.error.is_none(){input class="ui-file-input__input" type="file" multiple[s.multiple] data-ui-file-input[s.delegated] aria-label=(&s.input_label);}
    button type="button" class="ui-button ui-file-input__button" disabled[s.disabled || s.error.is_some()]{(&s.button)}
    input class=(if s.error.is_some(){"ui-file-input__display ui-input--error"}else if s.disabled{"ui-file-input__display"}else{"ui-file-input__display ui-file-input__display--medium"}) value=(&s.display) data-file-input-label[s.delegated] aria-invalid=[s.error.as_ref().map(|_|"true")] readonly[!s.disabled] disabled[s.disabled] aria-label=(&s.display_label);
    }@if let Some(error)=&s.error{span class="error-message"{(error)}}}}
});
#[derive(Clone, Copy)]
pub enum ProgressKind {
    Small,
    Medium,
    Large,
    Memory,
    Swap,
    Process,
    Disk,
}
impl ProgressKind {
    fn name(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Memory => "memory",
            Self::Swap => "swap",
            Self::Process => "process",
            Self::Disk => "disk",
        }
    }
    fn meter(self) -> bool {
        matches!(self, Self::Memory | Self::Swap | Self::Process | Self::Disk)
    }
}
#[derive(Clone)]
pub struct ProgressBar {
    pub kind: ProgressKind,
    pub label: String,
    pub right: Option<String>,
    pub aria_label: String,
    pub value: Option<u8>,
    pub fill_text: Option<String>,
    pub styled_labels: bool,
    pub default_fill: bool,
    pub hx: Hx,
}
renderer!(ProgressBar, s, {
    let fill = if s.kind.meter() {
        classes("ui-progress-bar__fill", &[Some(s.kind.name())])
    } else {
        classes(
            "ui-progress-bar__fill",
            &[s.default_fill.then_some("default")],
        )
    };
    html! {div class=(classes("ui-progress-bar",&[Some(s.kind.name()),s.value.is_none().then_some("indeterminate")])){div class="ui-progress-bar__labels"{span class=[s.styled_labels.then_some("ui-progress-bar__label")]{(&s.label)}@if let Some(right)=&s.right{span class=[s.styled_labels.then_some("ui-progress-bar__label-right")]{(right)}}}div class="ui-progress-bar__container" role=(if s.kind.meter(){"meter"}else{"progressbar"}) aria-label=(&s.aria_label) aria-valuemin=[s.value.map(|_|0)] aria-valuemax=[s.value.map(|_|100)] aria-valuenow=[s.value]{div class=(fill) style=[s.value.map(|v|format!("width:{}%",v.min(100)))]{@if let Some(text)=&s.fill_text{span class="ui-progress-bar__text"{(text)}}}}}}
});
#[derive(Clone)]
pub enum RowKind {
    Info {
        icon: String,
        name: String,
        detail: Option<String>,
        badges: Vec<Badge>,
    },
    Selected {
        checked: bool,
        text: String,
        badge: Badge,
    },
    Clickable {
        text: String,
        hint: String,
    },
    Error {
        text: String,
        badge: Badge,
    },
}
#[derive(Clone)]
pub struct RowInfoTile {
    pub kind: RowKind,
    pub hx: Hx,
}
renderer!(RowInfoTile, s, {
    match &s.kind {
        RowKind::Info {
            icon,
            name,
            detail,
            badges,
        } => {
            html! {div class="row-info-tile"{span{(icon)}strong{(name)}@if let Some(detail)=detail{span{(detail)}}@for badge in badges{(badge)}}}
        }
        RowKind::Selected {
            checked,
            text,
            badge,
        } => {
            html! {label class="row-info-tile row-info-tile--selected"{input type="checkbox" checked[*checked];(text)" "(badge)}}
        }
        RowKind::Clickable { text, hint } => {
            html! {button class="row-info-tile"{(text)" "span{(hint)}}}
        }
        RowKind::Error { text, badge } => {
            html! {div class="row-info-tile row-info-tile--error"{(text)" "(badge)}}
        }
    }
});
#[derive(Clone, Copy)]
pub enum TextBoxKind {
    Log,
    Code,
    Terminal,
}
#[derive(Clone)]
pub struct TextBox {
    pub kind: TextBoxKind,
    pub title: Option<String>,
    pub content: String,
    pub empty: bool,
    pub copy_label: Option<String>,
    pub refresh_label: Option<String>,
    pub delegated: bool,
    pub hx: Hx,
}
renderer!(TextBox, s, {
    html! {div class=(format!("ui-text-box ui-text-box--{} ui-text-box--monospace",match s.kind{TextBoxKind::Log=>"log",TextBoxKind::Code=>"code",TextBoxKind::Terminal=>"terminal"})){
        @if let Some(title)=&s.title{header class="ui-text-box__header"{span class="ui-text-box__header-title"{(title)}span class="ui-text-box__header-actions"{@if let Some(label)=&s.copy_label{button class="ui-button ui-button--small" data-copy-textbox{(label)}}@if let Some(label)=&s.refresh_label{button class="ui-button ui-button--secondary ui-button--small" data-refresh-log{(label)}}}}}
        div class="ui-text-box__container"{@if s.empty{div class="ui-text-box__content"{em{(&s.content)}}}@else{pre class="ui-text-box__content" data-log-content[s.delegated]{(&s.content)}}}
    }}
});
#[derive(Clone, Default)]
pub struct Modal {
    pub title: String,
    pub title_id: String,
    pub body: String,
    pub cancel: String,
    pub confirm: String,
    pub close_label: String,
    pub hidden: bool,
    pub hx: Hx,
}
renderer!(Modal, s, {
    html! {div class="modal-overlay" data-ux-modal-demo-backdrop aria-hidden=(s.hidden.to_string()){section class="modal medium" role="dialog" aria-modal="true" aria-labelledby=(&s.title_id) data-ux-modal-demo-window{button type="button" class="modal-close" data-ux-modal-close aria-label=(&s.close_label){"×"}h4 class="modal-title" id=(&s.title_id) data-ux-modal-demo-title{(&s.title)}div class="modal-content"{p data-ux-modal-demo-body{(&s.body)}}div class="modal-buttons"{button type="button" class="ui-button ui-button--secondary" data-ux-modal-close{(&s.cancel)}button type="button" class="ui-button" data-ux-modal-close{(&s.confirm)}}}}}
});
#[derive(Clone, Default)]
pub struct Tab {
    pub label: String,
    pub icon: Option<String>,
    pub active: bool,
    pub disabled: bool,
    pub hidden: bool,
    pub admin_only: bool,
    pub size: Option<Size>,
    pub depth: Option<u8>,
    pub visibility: Option<VisibilityToggle>,
    pub starred: Option<bool>,
    pub star_label: Option<String>,
    pub hx: Hx,
}
impl Tab {
    fn content(&self) -> Markup {
        html! {span class="ui-tab__content"{@if let Some(icon)=&self.icon{span class="ui-tab__icon"{(icon)}}span class="ui-tab__label"{(&self.label)}}}
    }
}
renderer!(Tab, s, {
    let mut class = "ui-tab".to_string();
    if s.active {
        class.push_str(" ui-tab--active active");
    }
    if s.hidden {
        class.push_str(" ui-tab--hidden");
    }
    if s.admin_only {
        class.push_str(" ui-tab--admin-only");
    }
    if let Some(size) = s.size {
        class.push_str(&format!(" ui-tab--{}", size.name()));
    }
    if s.hidden || s.admin_only || s.visibility.is_some() || s.starred.is_some() {
        html! {div class=(class) data-visibility-specimen[s.visibility.is_some() && s.starred.is_none()] data-visible=[s.visibility.as_ref().filter(|_|s.starred.is_none()).map(|v|v.visible.to_string())] data-has-both-icons=[(s.visibility.is_some() && s.starred.is_some()).then_some("true")]{
            @if let Some(visibility)=&s.visibility{div class="ui-tab__visibility-column"{(visibility)}}
            (s.content())
            @if let Some(starred)=s.starred{div class="ui-tab__star-column"{button type="button" class=(if starred{"ui-tab__star-button ui-tab__star-button--starred fas fa-star"}else{"ui-tab__star-button far fa-star"}) aria-pressed=(starred.to_string()) aria-label=[s.star_label.as_deref()] title=[s.star_label.as_deref()] {}}}
        }}
    } else {
        html! {button type="button" class=(class) disabled[s.disabled] data-depth=[s.depth]{(s.content())}}
    }
});
#[derive(Clone, Default)]
pub struct Tabs {
    pub tabs: Vec<Tab>,
    pub hx: Hx,
}
renderer!(Tabs, s, {
    html! {div class="ui-tab-group"{@for tab in &s.tabs{(tab)}}}
});
