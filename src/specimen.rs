//! Additional stateful showcase compositions using delegated host controls.
use crate::{html, Markup, Render};
use crate::ux::VisibilityToggle;

/// A labelled checked or unchecked control in a device group.
#[derive(Clone, Debug)]
pub struct DriveChoice { pub label: String, pub checked: bool }
/// A device name and its grouped drive choices.
#[derive(Clone, Debug)]
pub struct DeviceChoices { pub device: String, pub choices: Vec<DriveChoice> }
impl Render for DeviceChoices {
    fn render(&self) -> Markup { html!{div class="device-control" {
        span class="device-name" {(&self.device)}
        @for choice in &self.choices {label class="drive-checkbox" {input type="checkbox" checked[choice.checked]; (&choice.label)}}
    }} }
}

/// One visible or dimmed element with a delegated eye control.
#[derive(Clone, Debug)]
pub struct VisibilityCard { pub title: String, pub visible: bool, pub control_label: String, pub state_label: String }
impl Render for VisibilityCard {
    fn render(&self) -> Markup { let visible=self.visible.to_string(); html!{
        article class="showcase-item visibility-element-card" data-visibility-specimen data-visible=(visible){
            h4{(&self.title)}
            (VisibilityToggle{visible:self.visible,label:self.control_label.clone(),..VisibilityToggle::default()})
            p data-visibility-state-label{(&self.state_label)}
        }
    } }
}

/// A visibility tab whose eye control is managed by the host.
#[derive(Clone, Debug)]
pub struct VisibilityTab { pub label: String, pub visible: bool, pub active: bool, pub control_label: String }
/// A group comparing ordinary visibility and an admin-only tab.
#[derive(Clone, Debug)]
pub struct VisibilityTabSet { pub tabs: Vec<VisibilityTab>, pub admin_label: String }
impl Render for VisibilityTabSet {
    fn render(&self) -> Markup { html!{
        div class="ui-tab-group visibility-specimen-tabs" {
            @for tab in &self.tabs {
                div class=(if tab.active {"ui-tab ui-tab--active active"} else if !tab.visible {"ui-tab ui-tab--hidden"} else {"ui-tab"}) data-visibility-specimen data-visible=(tab.visible.to_string()) {
                    (VisibilityToggle{visible:tab.visible,label:tab.control_label.clone(),..VisibilityToggle::default()})
                    span class="ui-tab__content"{(&tab.label)}
                }
            }
            div class="ui-tab ui-tab--admin-only" data-visibility-admin-only {span class="ui-tab__content"{(&self.admin_label)}}
        }
    }}
}

/// A local portal-removal specimen; the host owns the delegated interaction.
/// Omits inline positioning; layout is determined by the host skin.
#[derive(Clone, Debug)]
pub struct PortalRemoveSimulator {
    pub name: String,
    pub description: String,
    pub visible: bool,
    pub visibility_label: String,
    pub delete_label: String,
}
impl Render for PortalRemoveSimulator {
    fn render(&self) -> Markup {
        let visible = self.visible.to_string();
        let eye = if self.visible { "fas fa-eye" } else { "fas fa-eye-slash" };
        html! { div class="portals-grid" data-test-portal-remove-simulator {
            div class="portal-element" data-portal-element data-visible=(&visible) {
                button type="button" class="visibility-toggle ui-visibility-toggle" data-visible=(&visible) aria-pressed=(&visible) aria-label=(&self.visibility_label) {
                    i class=(eye) aria-hidden="true" {}
                }
                button type="button" class="delete-portal-button" data-test-portal-delete data-portal-name=(&self.name) title=(&self.delete_label) aria-label=(&self.delete_label) {
                    i class="fas fa-trash" aria-hidden="true" {}
                }
                article class="portal-card up" data-portal-card data-portal-name=(&self.name) role="group" aria-label=(&self.name) {
                    div class="portal-card-face" { div class="portal-card-header" {
                        div class="portal-icon" aria-hidden="true" { i class="fas fa-flask" {} }
                        h4 class="portal-name" { (&self.name) }
                        p class="portal-description" { (&self.description) }
                    } }
                }
            }
        } }
    }
}

/// Selects a known descriptive specimen and its delegated catalog markers.
#[derive(Clone, Copy, Debug)]
pub enum VisibilityNoteKind { ElementsChanged, LiveMorph }
/// A description of how visibility updates are presented by the host.
#[derive(Clone)]
pub struct VisibilityNote { pub kind: VisibilityNoteKind, pub title: String, pub content: Markup }
impl Render for VisibilityNote {
    fn render(&self) -> Markup { match self.kind {
        VisibilityNoteKind::ElementsChanged=>html!{article class="showcase-item" data-catalog-specimen="elements-changed-pull" data-composable="elements-changed-pull" data-pulse-topic="elements.changed" {h4{(&self.title)} p{(self.content.clone())}}},
        VisibilityNoteKind::LiveMorph=>html!{article class="showcase-item" data-catalog-specimen="live-pane-dom-morph" data-vis-scar="VIS-004" {h4{(&self.title)} p{(self.content.clone())}}},
    }}
}

/// A loading frame with delegated state toggle and live result.
#[derive(Clone)]
pub struct LoadingFrame { pub toggle_label: String, pub result: String, pub loading: String, pub spinner: Markup }
impl Render for LoadingFrame { fn render(&self) -> Markup {html!{
    button type="button" class="ui-button ui-button--secondary" data-loading-spinner-toggle aria-pressed="true"{(&self.toggle_label)}
    p data-loading-spinner-result aria-live="polite"{(&self.result)}
    div class="network-loading" data-loading-spinner-frame data-loading-spinner-state="loading"{(self.spinner.clone()) p{(&self.loading)}}
}} }
/// A full-view loading composition.
#[derive(Clone)]
pub struct LoadingView { pub spinner: Markup, pub message: String }
impl Render for LoadingView { fn render(&self) -> Markup {html!{div class="stats-loading" data-loading-spinner-full-view{(self.spinner.clone()) div class="loading-message"{(&self.message)}}}} }
/// A busy action with an embedded, non-announced spinner.
#[derive(Clone)]
pub struct LoadingButton { pub label: String, pub spinner: Markup }
impl Render for LoadingButton { fn render(&self) -> Markup {html!{button type="button" class="ui-button ui-button--primary" aria-busy="true"{(&self.label) " " (self.spinner.clone())}}} }

/// A scaffold catalog scoped to one guest and version.
#[derive(Clone)]
pub struct ScaffoldGrid { pub guest: String, pub version: String, pub content: Markup }
impl Render for ScaffoldGrid { fn render(&self) -> Markup {html!{div class="showcase-grid" data-scaffold-root data-guest-id=(&self.guest) data-scaffold-version=(&self.version){(self.content.clone())}}} }

/// Named progress specimen marker.
#[derive(Clone, Copy, Debug)]
pub enum ProgressGroupKind { Basic, Sizes, Variants, Indeterminate }
/// A labelled progress specimen with its host marker.
#[derive(Clone)]
pub struct ProgressGroup { pub kind: ProgressGroupKind, pub title: String, pub content: Markup }
impl Render for ProgressGroup { fn render(&self) -> Markup { let c=self.content.clone(); match self.kind {
    ProgressGroupKind::Basic=>html!{div class="showcase-item" data-test-library-progress{h4{(&self.title)} (c)}},
    ProgressGroupKind::Sizes=>html!{div class="showcase-item" data-test-progress-sizes{h4{(&self.title)} (c)}},
    ProgressGroupKind::Variants=>html!{div class="showcase-item" data-test-progress-variants{h4{(&self.title)} (c)}},
    ProgressGroupKind::Indeterminate=>html!{div class="showcase-item" data-test-progress-indeterminate{h4{(&self.title)} (c)}},
}} }

/// A breadcrumb readout and its delegated reset action.
#[derive(Clone, Debug)]
pub struct BreadcrumbReadout { pub path: String, pub reset_label: String }
impl Render for BreadcrumbReadout { fn render(&self) -> Markup {html!{
    p{"Current path: " code data-breadcrumb-path{(&self.path)}}
    button type="button" class="ui-button ui-button--secondary ui-button--small" data-reset-breadcrumbs{(&self.reset_label)}
}} }
/// A file-selection readout and disabled-until-ready delegated action.
#[derive(Clone, Debug)]
pub struct FileSelectionReadout { pub message: String, pub submit_label: String }
impl Render for FileSelectionReadout { fn render(&self) -> Markup {html!{
    p class="muted" data-test-file-state{(&self.message)}
    button type="button" class="ui-button ui-button--primary ui-button--small" data-test-file-submit disabled{(&self.submit_label)}
}} }

/// Labels for the two ends of a slider.
#[derive(Clone, Debug)]
pub struct SliderLabels { pub start: String, pub end: String }
impl Render for SliderLabels { fn render(&self) -> Markup {html!{div class="slider-labels"{span{(&self.start)} span{(&self.end)}}}} }
/// Live address readouts for a reservation boundary slider.
#[derive(Clone, Debug)]
pub struct AddressBoundaryReadouts { pub reserved_prefix: String, pub reserved_end: String, pub pool_prefix: String, pub pool_start: String, pub pool_suffix: String }
impl Render for AddressBoundaryReadouts { fn render(&self) -> Markup {html!{
    div{"Reserved Range: " code{(&self.reserved_prefix) span data-dhcp-example-reserved-end{(&self.reserved_end)}}}
    div{"Pool Range: " code{(&self.pool_prefix) span data-dhcp-example-pool-start{(&self.pool_start)} (&self.pool_suffix)}}
}} }

/// Readout for the selected weekly calendar value.
#[derive(Clone, Debug)]
pub struct CalendarReadout { pub value: String }
impl Render for CalendarReadout { fn render(&self) -> Markup {html!{p class="muted" data-ui-calendar-output{(&self.value)}}} }
/// Readout for a selected time value.
#[derive(Clone, Debug)]
pub struct TimeReadout { pub before: String, pub value: String, pub after: String }
impl Render for TimeReadout { fn render(&self) -> Markup {html!{p{(&self.before) span data-ui-time-output{(&self.value)} (&self.after)}}} }

/// Marked container for a monthly calendar specimen.
#[derive(Clone)]
pub struct MonthlyCalendarItem { pub title: String, pub content: Markup }
impl Render for MonthlyCalendarItem { fn render(&self) -> Markup {html!{div class="showcase-item" data-ui-calendar-monthly{h4{(&self.title)} (self.content.clone())}}} }
/// Marked container for a local network address specimen.
#[derive(Clone)]
pub struct AddressCalendarItem { pub title: String, pub description: Markup, pub content: Markup }
impl Render for AddressCalendarItem { fn render(&self) -> Markup {html!{div class="showcase-item" data-catalog-specimen="lan-ip-octet-calendar" data-test-lan-ip-calendar{h4{(&self.title)} p{(self.description.clone())} (self.content.clone())}}} }
