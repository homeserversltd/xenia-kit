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
    pub vals: Option<String>,
    pub include: Option<String>,
    pub confirm: Option<String>,
    pub indicator: Option<String>,
    pub disabled_elt: Option<String>,
}
impl Hx {
    fn attributes(&self) -> String {
        let mut attrs = String::new();
        for (key, value) in [
            ("hx-get", self.get.as_deref()),
            ("hx-post", self.post.as_deref()),
            ("hx-target", self.target.as_deref()),
            ("hx-swap", self.swap.map(Swap::name)),
            ("hx-trigger", self.trigger.as_deref()),
            ("hx-vals", self.vals.as_deref()),
            ("hx-include", self.include.as_deref()),
            ("hx-confirm", self.confirm.as_deref()),
            ("hx-indicator", self.indicator.as_deref()),
            ("hx-disabled-elt", self.disabled_elt.as_deref()),
        ] {
            if let Some(value) = value {
                attrs.push_str(&format!(" {key}=\"{}\"", html! {(value)}.0));
            }
        }
        attrs
    }

    fn apply_at(&self, mut markup: Markup, tag: Option<&str>) -> Markup {
        let attrs = self.attributes();
        if !attrs.is_empty() {
            let at = if let Some(tag) = tag {
                let marker = format!("<{tag}");
                markup.0.find(&marker).and_then(|start| {
                    markup.0[start..]
                        .find('>')
                        .map(|relative_end| start + relative_end)
                })
            } else {
                markup.0.find('>')
            };
            if let Some(at) = at {
                markup.0.insert_str(at, &attrs);
            }
        }
        markup
    }

    pub(crate) fn apply(&self, markup: Markup) -> Markup {
        self.apply_at(markup, None)
    }

    pub(crate) fn apply_to_control(&self, markup: Markup, tag: &str) -> Markup {
        self.apply_at(markup, Some(tag))
    }
}

#[derive(Clone)]
pub struct Form {
    pub method_post: bool,
    pub hx: Hx,
    pub body: Markup,
}
impl Render for Form {
    fn render(&self) -> Markup {
        self.hx.apply(html! {
            form method=(if self.method_post { "post" } else { "get" }) {
                (self.body.clone())
            }
        })
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
pub enum ToastKind {
    Info,
    Success,
    Warning,
    Error,
}
impl ToastKind {
    fn name(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct ToastTimeout(u64);
impl ToastTimeout {
    pub fn new(milliseconds: u64) -> Result<Self, String> {
        if !(1..=2_147_483_647).contains(&milliseconds) {
            return Err("toast timeout must be between 1 and 2147483647 milliseconds".into());
        }
        Ok(Self(milliseconds))
    }

    pub fn milliseconds(self) -> u64 {
        self.0
    }
}
#[derive(Clone, Debug)]
pub struct Toast {
    pub kind: ToastKind,
    pub message: String,
    pub timeout: Option<ToastTimeout>,
}
impl Toast {
    pub fn new(kind: ToastKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            timeout: None,
        }
    }

    pub fn with_timeout(mut self, timeout: ToastTimeout) -> Self {
        self.timeout = Some(timeout);
        self
    }
}
impl Render for Toast {
    fn render(&self) -> Markup {
        if self.message.is_empty() {
            return html! {};
        }
        html! {
            div hx-swap-oob="beforeend:[data-coronatio-toast-stack]" {
                span hidden data-coronatio-toast-request data-toast-kind=(self.kind.name()) data-toast-timeout-ms=[self.timeout.map(ToastTimeout::milliseconds)] {
                    (&self.message)
                }
            }
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
macro_rules! control_renderer {
    ($t:ty,$s:ident,$body:block,$tag:literal) => {
        impl Render for $t {
            fn render(&self) -> Markup {
                let $s = self;
                let body: Markup = $body;
                $s.hx.apply_to_control(body, $tag)
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
    Markup(Markup),
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
            Self::Markup(markup) => markup.clone(),
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
    pub header_badges: Vec<Badge>,
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
            @if s.header.is_some() || !s.header_badges.is_empty() {
                div class="ui-card__header"{
                    @if let Some(header)=&s.header {(header)}
                    @for badge in &s.header_badges {(badge)}
                }
            }
            @if s.body_wrapper || s.header.is_some() || !s.header_badges.is_empty() || !s.footer.is_empty(){div class="ui-card__body"{(&s.body)}}@else{(&s.body)}
            @if !s.footer.is_empty(){div class="ui-card__footer"{@for action in &s.footer{(action)}}}
        }}
    }
});
#[derive(Clone, Default)]
pub struct Toggle {
    pub label: Option<String>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub size: Option<Size>,
    pub checked: bool,
    pub disabled: bool,
    pub aria_label: Option<String>,
    pub delegated: bool,
    pub hx: Hx,
}
control_renderer!(
    Toggle,
    s,
    {
        html! {label class=(classes("ui-toggle",&[s.size.map(Size::name),s.disabled.then_some("disabled")])) {span class="ui-toggle__switch" {input class="ui-toggle__input" type="checkbox" name=[s.name.as_deref()] value=[s.value.as_deref()] checked[s.checked] disabled[s.disabled] aria-label=[s.aria_label.as_deref()] data-ui-toggle[s.delegated];span class="ui-toggle__slider"{}} @if let Some(label)=&s.label {span class="ui-toggle__label"{(label)}}}}
    },
    "input"
);
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
    pub name: Option<String>,
    pub value: Option<String>,
    pub size: Option<Size>,
    pub checked: bool,
    pub disabled: bool,
    pub delegated: bool,
    pub hx: Hx,
}
control_renderer!(
    Checkbox,
    s,
    {
        html! {label class=(classes("ui-checkbox",&[s.size.map(Size::name)])){input type="checkbox" name=[s.name.as_deref()] value=[s.value.as_deref()] checked[s.checked] disabled[s.disabled] data-ui-checkbox[s.delegated];(&s.label)}}
    },
    "input"
);
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
    pub name: Option<String>,
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
control_renderer!(
    Input,
    s,
    {
        let field = html! {input class=(classes("ui-input",&[s.error.as_ref().map(|_|"error"),s.display.then_some("display"),s.size.map(Size::name)])) type=[s.kind.as_deref()] name=[s.name.as_deref()] required[s.required] placeholder=[s.placeholder.as_deref()] value=[s.value.as_deref()] disabled[s.disabled] readonly[s.readonly] data-ui-input[s.delegated];};
        if let Some(label) = &s.label {
            html! {label class="ui-input-label"{(label)(field)@if let Some(error)=&s.error{span class="ui-input__error"{(error)}}}}
        } else {
            field
        }
    },
    "input"
);
#[derive(Clone, Default)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub selected: bool,
    pub disabled: bool,
}
#[derive(Clone, Default)]
pub struct Select {
    pub label: String,
    pub options: Vec<String>,
    pub choices: Vec<SelectOption>,
    pub name: Option<String>,
    pub required: bool,
    pub disabled: bool,
    pub delegated: bool,
    pub error: Option<String>,
    pub hx: Hx,
}
control_renderer!(
    Select,
    s,
    {
        html! {label class=(if s.error.is_some(){"ui-select-label ui-select--error"}else{"ui-select-label"}){(&s.label)select class="ui-select" name=[s.name.as_deref()] data-ui-select[s.delegated] required[s.required] disabled[s.disabled]{@if s.choices.is_empty(){@for option in &s.options{option{(option)}}}@else{@for choice in &s.choices{option value=(&choice.value) selected[choice.selected] disabled[choice.disabled]{(&choice.label)}}}}@if let Some(error)=&s.error{span class="ui-input__error"{(error)}}}}
    },
    "select"
);
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
            html! {
                button class="row-info-tile" {
                    (text)
                    " "
                    span { (hint) }
                }
            }
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

#[derive(Clone, Copy, Debug)]
pub enum SkeletonLine {
    Full,
    Medium,
    Short,
}

#[derive(Clone)]
pub enum SkeletonChild {
    Row(Vec<SkeletonChild>),
    Stack(Vec<SkeletonChild>),
    Line(SkeletonLine),
    Avatar,
    Block,
    Slot(Slot),
    ReadySlot(ReadySlot),
}

#[derive(Clone)]
pub struct Skeleton {
    guest_id: String,
    version: String,
    label: Option<String>,
    children: Vec<SkeletonChild>,
}

impl Skeleton {
    pub fn new(guest_id: impl Into<String>, version: impl Into<String>) -> Result<Self, String> {
        let guest_id = guest_id.into();
        if !valid_scaffold_guest_id(&guest_id) {
            return Err("guest id must match ^[a-z0-9][a-z0-9-]*$".into());
        }
        let version = version.into();
        if version.is_empty() {
            return Err("skeleton version must not be empty".into());
        }
        Ok(Self {
            guest_id,
            version,
            label: None,
            children: Vec::new(),
        })
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_children(mut self, children: Vec<SkeletonChild>) -> Self {
        self.children = children;
        self
    }
}

fn valid_scaffold_guest_id(guest_id: &str) -> bool {
    let mut chars = guest_id.chars();
    matches!(chars.next(), Some(first) if first.is_ascii_lowercase() || first.is_ascii_digit())
        && chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
}

impl Render for Skeleton {
    fn render(&self) -> Markup {
        html! {
            div class="ui-skeleton" data-scaffold-root data-guest-id=(&self.guest_id)
                data-scaffold-version=(&self.version) aria-label=[self.label.as_deref()] {
                @for child in &self.children {
                    (render_skeleton_child(child, &self.guest_id))
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SlotTimeout(u32);

impl SlotTimeout {
    pub fn new(milliseconds: u32) -> Result<Self, String> {
        if !(1..=30_000).contains(&milliseconds) {
            return Err("slot timeout must be between 1 and 30000 milliseconds".into());
        }
        Ok(Self(milliseconds))
    }
}

#[derive(Clone)]
pub struct Slot {
    name: String,
    parts: Vec<String>,
    content: Markup,
    placeholder: Option<String>,
    fill: Option<String>,
    timeout: Option<SlotTimeout>,
    children: Vec<SkeletonChild>,
}

impl Slot {
    pub fn new(
        name: impl Into<String>,
        part: impl AsRef<str>,
        content: Markup,
    ) -> Result<Self, String> {
        Self::from_segments(name, std::iter::once(part.as_ref().to_owned()), content)
    }

    pub fn from_segments(
        name: impl Into<String>,
        parts: impl IntoIterator<Item = impl AsRef<str>>,
        content: Markup,
    ) -> Result<Self, String> {
        let name = valid_scaffold_slot_name(name.into())?;
        let parts = parts
            .into_iter()
            .map(|part| part.as_ref().to_owned())
            .collect::<Vec<_>>();
        if parts.is_empty() {
            return Err("slot path must contain at least one segment".into());
        }
        if parts.iter().any(|part| !valid_scaffold_path_segment(part)) {
            return Err("slot path segments must be non-empty ASCII unreserved path segments".into());
        }
        Ok(Self {
            name,
            parts,
            content,
            placeholder: None,
            fill: None,
            timeout: None,
            children: Vec::new(),
        })
    }

    pub fn ready(name: impl Into<String>, content: Markup) -> Result<ReadySlot, String> {
        Ok(ReadySlot {
            name: valid_scaffold_slot_name(name.into())?,
            content,
            placeholder: None,
            fill: None,
        })
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_fill(mut self, fill: impl Into<String>) -> Self {
        self.fill = Some(fill.into());
        self
    }

    pub fn with_timeout(mut self, timeout: SlotTimeout) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_children(mut self, children: Vec<SkeletonChild>) -> Self {
        self.children = children;
        self
    }

    fn render_pending(&self, guest_id: &str) -> Markup {
        let href = format!("/api/tabs/{guest_id}/{}", self.parts.join("/"));
        let request = self
            .timeout
            .map(|timeout| format!(r#"{{"timeout":{}}}"#, timeout.0));
        html! {
            div class="ui-slot" data-scaffold-slot=(&self.name)
                data-scaffold-placeholder=[self.placeholder.as_deref()]
                data-scaffold-fill=[self.fill.as_deref()]
                data-scaffold-state="pending" aria-busy="true"
                hx-get=(href.as_str()) hx-trigger="load" hx-request=[request.as_deref()] {
                (self.content.clone())
                @for child in &self.children {
                    (render_skeleton_child(child, guest_id))
                }
            }
        }
    }
}

fn valid_scaffold_slot_name(name: String) -> Result<String, String> {
    if name.trim().is_empty() {
        return Err("slot name must not be empty or whitespace-only".into());
    }
    Ok(name)
}

fn valid_scaffold_path_segment(segment: &str) -> bool {
    !segment.is_empty()
        && segment != "."
        && segment != ".."
        && !segment.contains('/')
        && !segment.contains('\\')
        && segment.as_bytes().iter().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(*byte, b'-' | b'_' | b'.' | b'~')
        })
}

#[derive(Clone)]
pub struct ReadySlot {
    name: String,
    content: Markup,
    placeholder: Option<String>,
    fill: Option<String>,
}

impl ReadySlot {
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_fill(mut self, fill: impl Into<String>) -> Self {
        self.fill = Some(fill.into());
        self
    }
}

impl Render for ReadySlot {
    fn render(&self) -> Markup {
        html! {
            div class="ui-slot" data-scaffold-slot=(&self.name)
                data-scaffold-placeholder=[self.placeholder.as_deref()]
                data-scaffold-fill=[self.fill.as_deref()]
                data-scaffold-state="ready" aria-busy="false" {
                (self.content.clone())
            }
        }
    }
}

fn render_skeleton_child(child: &SkeletonChild, guest_id: &str) -> Markup {
    match child {
        SkeletonChild::Row(children) => html! {
            div class="ui-skeleton__row" {
                @for child in children {
                    (render_skeleton_child(child, guest_id))
                }
            }
        },
        SkeletonChild::Stack(children) => html! {
            div class="ui-skeleton__stack" {
                @for child in children {
                    (render_skeleton_child(child, guest_id))
                }
            }
        },
        SkeletonChild::Line(SkeletonLine::Full) => {
            html! { span class="ui-skeleton__line" {} }
        }
        SkeletonChild::Line(SkeletonLine::Medium) => {
            html! { span class="ui-skeleton__line ui-skeleton__line--medium" {} }
        }
        SkeletonChild::Line(SkeletonLine::Short) => {
            html! { span class="ui-skeleton__line ui-skeleton__line--short" {} }
        }
        SkeletonChild::Avatar => html! { span class="ui-skeleton__avatar" {} },
        SkeletonChild::Block => html! { div class="ui-skeleton__block" {} },
        SkeletonChild::Slot(slot) => slot.render_pending(guest_id),
        SkeletonChild::ReadySlot(slot) => slot.render(),
    }
}
