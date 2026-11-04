//! Composable layouts for a cartridge using the crown's existing skin.
//!
//! These builders emit markup only. Text and attribute values are escaped;
//! interactivity is supplied by the host's delegated controls.
use crate::{html, Markup, Render};

/// One tab in a scoped, delegated tab strip.
#[derive(Clone, Debug)]
pub struct TabChoice {
    pub id: String,
    pub label: String,
    pub active: bool,
}

/// The three supported levels of tab nesting.
#[derive(Clone, Copy, Debug)]
pub enum TabDepth { Document, Panel, Category }
impl TabDepth {
    fn number(self) -> u8 { match self { Self::Document => 1, Self::Panel => 2, Self::Category => 3 } }
}

/// A delegated tab strip. Panel is a pane section; Category is a category.
#[derive(Clone, Debug)]
pub struct TabStrip {
    pub label: String,
    pub depth: TabDepth,
    pub categories: bool,
    pub tabs: Vec<TabChoice>,
}

impl Render for TabStrip {
    fn render(&self) -> Markup {
        html! {
            div class="ui-tab-group" role="tablist" aria-label=(&self.label) {
                @for tab in &self.tabs {
                    button type="button"
                        class=(if tab.active { "ui-tab ui-tab--active active" } else if self.categories { "ui-tab " } else { "ui-tab" })
                        data-tab-id=(&tab.id)
                        data-category-chip=[self.categories.then_some(tab.id.as_str())]
                        aria-selected=(tab.active.to_string()) data-depth=(self.depth.number()) {
                        span class="ui-tab__content" { span class="ui-tab__label" { (&tab.label) } }
                    }
                }
            }
        }
    }
}

/// Visual form of a named panel.
#[derive(Clone, Copy, Debug)]
pub enum PanelKind { Basic, Services, Configuration, Health, Animations }

/// A named panel in a scoped tab strip.
#[derive(Clone)]
pub struct Panel {
    pub id: String,
    pub active: bool,
    pub kind: PanelKind,
    pub catalog_specimen: Option<String>,
    pub content: Markup,
}

impl Render for Panel {
    fn render(&self) -> Markup {
        let base = match self.kind {
            PanelKind::Basic => "test-panel",
            PanelKind::Services => "test-panel test-services",
            PanelKind::Configuration => "test-panel test-config",
            PanelKind::Health => "test-panel test-health",
            PanelKind::Animations => "test-panel test-animations",
        };
        let class = if self.active { format!("{base} active") } else { base.to_owned() };
        html! {
            section class=(class) data-tab-panel=(&self.id)
                data-catalog-specimen=[self.catalog_specimen.as_deref()]
                data-animation-lab[matches!(self.kind, PanelKind::Animations)] { (self.content.clone()) }
        }
    }
}

/// A tablet with a scoped tab strip and the corresponding panels.
#[derive(Clone)]
pub struct Tablet {
    pub scope: String,
    pub tabs: TabStrip,
    pub panels: Markup,
}
impl Render for Tablet {
    fn render(&self) -> Markup {
        html! { div class="test-tablet" data-tab-scope=(&self.scope) {
            (&self.tabs) div class="test-tablet-content" { (self.panels.clone()) }
        } }
    }
}

/// The scoped showcase holding category tabs and category panels.
#[derive(Clone)]
pub struct Showcase {
    pub scope: String,
    pub identity: String,
    pub tabs: TabStrip,
    pub categories: Markup,
}
impl Render for Showcase {
    fn render(&self) -> Markup {
        html! { div class="component-showcase" data-tab-scope=(&self.scope) data-component-showcase=(&self.identity) {
            (&self.tabs) div class="showcase-content" { (self.categories.clone()) }
        } }
    }
}

/// The grid holding a collection of showcase items.
#[derive(Clone)]
pub struct ShowcaseGrid {
    pub content: Markup,
}
impl Render for ShowcaseGrid {
    fn render(&self) -> Markup { html! { div class="showcase-grid" { (self.content.clone()) } } }
}

/// A labelled item in a showcase grid.
#[derive(Clone)]
pub struct ShowcaseItem {
    pub title: String,
    pub content: Markup,
}
impl Render for ShowcaseItem {
    fn render(&self) -> Markup {
        html! { div class="showcase-item" { h4 { (&self.title) } (self.content.clone()) } }
    }
}

/// A category section belonging to a showcase tab strip.
#[derive(Clone)]
pub struct ShowcaseCategory {
    pub id: String,
    pub title: String,
    pub active: bool,
    pub content: Markup,
}
impl Render for ShowcaseCategory {
    fn render(&self) -> Markup {
        html! {
            section id=(format!("showcase-{}", self.id))
                class=(if self.active { "showcase-section active" } else { "showcase-section " })
                data-tab-panel=(&self.id) data-og-category-section=(&self.id) {
                h3 { (&self.title) } (self.content.clone())
            }
        }
    }
}

/// Shared heading for a subpanel. The optional trailing content can be a badge or action.
#[derive(Clone)]
pub struct SubpanelHeader {
    pub kicker: String,
    pub title: String,
    pub description: String,
    pub trailing: Markup,
}
impl Render for SubpanelHeader {
    fn render(&self) -> Markup {
        html! {
            header class="test-subpanel-header" {
                div { p class="test-kicker" { (&self.kicker) } h3 { (&self.title) } p { (&self.description) } }
                (self.trailing.clone())
            }
        }
    }
}

/// A toast-spawn button using the host's delegated notification control.
#[derive(Clone, Debug)]
pub struct ToastSpawn {
    pub label: String,
    pub message: String,
    pub kind: ToastSpawnKind,
}

/// Notification styles supported by the host.
#[derive(Clone, Copy, Debug)]
pub enum ToastSpawnKind { Info, Success, Warning, Error, Stack }
impl Render for ToastSpawn {
    fn render(&self) -> Markup {
        let (kind, tone) = match self.kind {
            ToastSpawnKind::Info => ("info", "primary"),
            ToastSpawnKind::Success => ("success", "success"),
            ToastSpawnKind::Warning => ("warning", "warning"),
            ToastSpawnKind::Error => ("error", "danger"),
            ToastSpawnKind::Stack => ("info", "secondary"),
        };
        html! {
            button type="button" class=(format!("ui-button ui-button--{tone}"))
                data-coronatio-toast-spawn data-toast-variant=(kind)
                data-toast-message=(&self.message) { (&self.label) }
        }
    }
}

/// Availability of a portal card.
#[derive(Clone, Copy, Debug)]
pub enum PortalState { Up, Down, Partial, Unknown }

/// Service portal face with host-owned controls supplied as composable markup.
#[derive(Clone)]
pub struct PortalCard {
    pub name: String,
    pub description: String,
    pub image: String,
    pub state: PortalState,
    pub details: Markup,
}
impl Render for PortalCard {
    fn render(&self) -> Markup {
        let state = match self.state { PortalState::Up => "up", PortalState::Down => "down", PortalState::Partial => "partial", PortalState::Unknown => "unknown" };
        html! {
            article class=(format!("portal-card {state}")) {
                div class="portal-card-face" {
                    div class="portal-card-header" {
                        img src=(&self.image) alt=(format!("{} icon", self.name)) class="portal-icon";
                        h4 class="portal-name" { (&self.name) }
                        p class="portal-description" { (&self.description) }
                    }
                    div class="portal-meta" { (self.details.clone()) }
                }
            }
        }
    }
}

/// Configuration card body shape.
#[derive(Clone, Copy, Debug)]
pub enum ConfigKind { Readback, Form, Table }

/// One configuration pattern card, with caller-composed body controls.
#[derive(Clone)]
pub struct ConfigPattern {
    pub pattern: String,
    pub marker: String,
    pub title: String,
    pub kind: ConfigKind,
    pub body: Markup,
}
impl Render for ConfigPattern {
    fn render(&self) -> Markup {
        let class = match self.kind { ConfigKind::Table => "ui-card test-config-pattern test-config-table-pattern", _ => "ui-card test-config-pattern" };
        html! {
            article class=(class) data-config-pattern=(&self.pattern) {
                div class="ui-card__header" { span { (&self.marker) } h4 { (&self.title) } }
                @match self.kind {
                    ConfigKind::Readback => { div class="ui-card__body" { (self.body.clone()) } }
                    ConfigKind::Form => { form class="ui-card__body test-config-form" { (self.body.clone()) } }
                    ConfigKind::Table => { (self.body.clone()) }
                }
            }
        }
    }
}

/// Availability of a health readout.
#[derive(Clone, Copy, Debug)]
pub enum HealthState { Healthy, Partial, Down, Unknown }

/// A labelled service health readout.
#[derive(Clone, Debug)]
pub struct HealthReadout {
    pub label: String,
    pub value: String,
    pub state: HealthState,
}
impl Render for HealthReadout {
    fn render(&self) -> Markup {
        let state = match self.state { HealthState::Healthy => "healthy", HealthState::Partial => "partial", HealthState::Down => "down", HealthState::Unknown => "unknown" };
        html! { article class=(format!("health-service-card {state}")) { h5 { (&self.label) } strong { (&self.value) } } }
    }
}

/// A grid of service portal cards and their add control.
#[derive(Clone)]
pub struct PortalGrid { pub content: Markup }
impl Render for PortalGrid {
    fn render(&self) -> Markup { html! { div class="portals-grid" data-test-services-grid { (self.content.clone()) } } }
}

/// A notice for a service managed by a script.
#[derive(Clone, Debug)]
pub struct ServiceNotice { pub title: String, pub text: String }
impl Render for ServiceNotice {
    fn render(&self) -> Markup { html! { div class="admin-controls script-management-notice" { span class="script-notice-text" title=(&self.title) { (&self.text) } } } }
}

/// The delegated portal-creation card.
#[derive(Clone, Debug)]
pub struct AddPortalCard {
    pub accessible_label: String,
    pub title: String,
    pub description: String,
}
impl Render for AddPortalCard {
    fn render(&self) -> Markup {
        html! { button type="button" class="portal-card add-portal-card" data-test-add-portal aria-label=(&self.accessible_label) {
            div class="portal-card-face" { div class="add-portal-content" {
                div class="add-portal-icon" { i class="fas fa-plus" {} }
                h3 class="add-portal-title" { (&self.title) }
                p class="add-portal-description" { (&self.description) }
            } }
        } }
    }
}

/// Service portal collection. Each card's surrounding element retains its own visibility and service kind.
#[derive(Clone)]
pub struct ServicePortal {
    pub visible: bool,
    pub service_type: String,
    pub card: PortalCard,
}
impl Render for ServicePortal {
    fn render(&self) -> Markup {
        html! { div class="portal-element" data-visible=(self.visible.to_string()) data-service-type=(&self.service_type) { (&self.card) } }
    }
}

/// An individual service action control, such as Start or Status.
#[derive(Clone, Debug)]
pub struct ServiceAction {
    pub label: String,
    pub title: String,
}
impl Render for ServiceAction {
    fn render(&self) -> Markup { html! { button title=(&self.title) { (&self.label) } } }
}

/// Rows of service actions inside a portal's metadata area.
#[derive(Clone, Default)]
pub struct ServiceActions {
    pub rows: Vec<Vec<ServiceAction>>,
}
impl Render for ServiceActions {
    fn render(&self) -> Markup {
        html! { div class="admin-controls" { @for row in &self.rows { div class="admin-controls-row" { @for action in row { (action) } } } } }
    }
}

/// A compact status dot: the meaning is selected by state, not by a caller's class name.
#[derive(Clone, Copy, Debug)]
pub enum StatusDot { Up, Down, Unknown }
impl Render for StatusDot {
    fn render(&self) -> Markup {
        let class = match self { Self::Up => "status-dot up", Self::Down => "status-dot down", Self::Unknown => "status-dot" };
        html! { span class=(class) {} }
    }
}

/// One upstream resolver option with a delegated value.
#[derive(Clone, Debug)]
pub struct ResolverOption {
    pub value: String,
    pub label: String,
    pub addresses: String,
}

/// Resolver choices, optional custom input, encrypted-DNS toggle and readout.
#[derive(Clone, Debug)]
pub struct UpstreamResolvers {
    pub providers: Vec<ResolverOption>,
    pub custom_label: String,
    pub custom_placeholder: String,
    pub encrypted_label: String,
    pub readout: String,
}
impl Render for UpstreamResolvers {
    fn render(&self) -> Markup {
        html! {
            div class="ui-upstream-resolver" {
                div class="ui-upstream-resolver__list" {
                    @for provider in &self.providers {
                        label class="ui-upstream-resolver__row" {
                            input type="checkbox" data-upstream-provider value=(&provider.value); " " (&provider.label) " " small { (&provider.addresses) }
                        }
                    }
                    label class="ui-upstream-resolver__row" { input type="checkbox" data-upstream-custom-toggle; " Add your own" }
                }
                label class="ui-upstream-resolver__custom" data-upstream-custom hidden {
                    (&self.custom_label) input class="ui-input" data-upstream-custom-addresses placeholder=(&self.custom_placeholder);
                }
                label class="ui-toggle ui-toggle--medium" {
                    span class="ui-toggle__switch" { input class="ui-toggle__input" type="checkbox" data-upstream-dot; span class="ui-toggle__slider" {} }
                    span class="ui-toggle__label" { (&self.encrypted_label) }
                }
                p class="ui-upstream-resolver__readout" data-upstream-readout aria-live="polite" { (&self.readout) }
            }
        }
    }
}

/// A resolver specimen's delegated scope and explanatory labels.
#[derive(Clone, Debug)]
pub struct ResolverSpecimen {
    pub title: String,
    pub description: String,
    pub controls: UpstreamResolvers,
}
impl Render for ResolverSpecimen {
    fn render(&self) -> Markup {
        html! { div class="showcase-item" data-catalog-specimen="upstream-resolver-checkbox-set" data-test-upstream-resolver {
            h4 { (&self.title) } p { (&self.description) } (&self.controls)
        } }
    }
}

/// Health viewport presentation.
#[derive(Clone, Copy, Debug)]
pub enum HealthViewKind { Summary, Services, Diagnostics }

/// A health viewport with a header and caller-composed readouts.
#[derive(Clone)]
pub struct HealthViewport {
    pub id: String,
    pub kind: HealthViewKind,
    pub marker: String,
    pub title: String,
    pub action: Markup,
    pub body: Markup,
}
impl Render for HealthViewport {
    fn render(&self) -> Markup {
        let class = match self.kind { HealthViewKind::Summary => "ui-card health-viewport health-summary", HealthViewKind::Services => "ui-card health-viewport", HealthViewKind::Diagnostics => "ui-card health-viewport health-diagnostics" };
        let body_class = if matches!(self.kind, HealthViewKind::Services) { "ui-card__body health-card-grid" } else { "ui-card__body" };
        html! {
            article class=(class) data-health-viewport=(&self.id) {
                div class="ui-card__header" { span { (&self.marker) } h4 { (&self.title) } (self.action.clone()) }
                div class=(body_class) { (self.body.clone()) }
            }
        }
    }
}

/// The status indicator used by health summaries.
#[derive(Clone, Copy, Debug)]
pub struct HealthDot(pub HealthState);
impl Render for HealthDot {
    fn render(&self) -> Markup {
        let state = match self.0 { HealthState::Healthy => "healthy", HealthState::Partial => "partial", HealthState::Down => "down", HealthState::Unknown => "unknown" };
        html! { span class=(format!("health-dot {state}")) {} }
    }
}

/// Delegated control that asks the host to stop motion.
#[derive(Clone, Debug)]
pub struct StillnessControl { pub label: String }
impl Render for StillnessControl {
    fn render(&self) -> Markup { html! { button type="button" class="ui-button ui-button--secondary" data-motion-stillness { (&self.label) } } }
}

/// One band in an animation showcase with a reusable title and composed stages.
#[derive(Clone)]
pub struct AnimationBand {
    pub id: String,
    pub title: String,
    pub description: String,
    pub content: Markup,
}
impl Render for AnimationBand {
    fn render(&self) -> Markup {
        html! { section class="animation-band" data-animation-band=(&self.id) {
            header { h3 { (&self.title) } p { (&self.description) } }
            (self.content.clone())
        } }
    }
}

/// An animation stage with a labelled object.
#[derive(Clone, Debug)]
pub struct MotionStage { pub label: String }
impl Render for MotionStage {
    fn render(&self) -> Markup { html! { div class="motion-stage" { span class="motion-jewel" { (&self.label) } } } }
}

/// Motion appearance for an animation specimen.
#[derive(Clone, Copy, Debug)]
pub enum MotionKind { Plain, Fade, Slide, Scale, Color, Height, ReducedSlide }

/// One labelled animation demonstration with a host-delegated play control.
#[derive(Clone)]
pub struct AnimationSpecimen {
    pub title: String,
    pub kind: MotionKind,
    pub hover: bool,
    pub catalog_id: Option<String>,
    pub description: Option<String>,
    pub stage: Markup,
    pub play_label: Option<String>,
}
impl Render for AnimationSpecimen {
    fn render(&self) -> Markup {
        let class = match self.kind {
            MotionKind::Plain => "animation-specimen", MotionKind::Fade => "animation-specimen motion-fade",
            MotionKind::Slide => "animation-specimen motion-slide", MotionKind::Scale => "animation-specimen motion-scale",
            MotionKind::Color => "animation-specimen motion-color", MotionKind::Height => "animation-specimen motion-height",
            MotionKind::ReducedSlide => "animation-specimen motion-slide motion-reduced-preview",
        };
        html! {
            article class=(class) data-hover-specimen[self.hover]
                data-animation-catalog-id=[self.catalog_id.as_deref()]
                data-motion-reduce-preview[matches!(self.kind, MotionKind::ReducedSlide)] {
                h4 { (&self.title) }
                @if let Some(description) = &self.description { p { (description) } }
                (self.stage.clone())
                @if let Some(label) = &self.play_label { button class="ui-button" data-animation-play { (label) } }
            }
        }
    }
}
