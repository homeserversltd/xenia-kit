//! Typed configuration and health readout parts.
use crate::{html, Markup, Render};
use crate::layout::{HealthDot, HealthState};

/// A labelled key and value in a configuration readback.
#[derive(Clone, Debug)]
pub struct ReadbackEntry { pub label: String, pub value: String }
/// A readback section with its name and entries.
#[derive(Clone, Debug)]
pub struct ReadbackSection { pub title: String, pub entries: Vec<ReadbackEntry> }
impl Render for ReadbackSection {
    fn render(&self) -> Markup { html! { section class="config-readback-section" { h5 { (&self.title) } dl { @for entry in &self.entries { div { dt { (&entry.label) } dd { (&entry.value) } } } } } } }
}

/// A choice in a configuration selector.
#[derive(Clone, Debug)]
pub struct ConfigChoice { pub label: String }
/// A compact configuration editor; the host owns the save action.
#[derive(Clone, Debug)]
pub struct ConfigEditor {
    pub name_label: String, pub name: String, pub selection_label: String,
    pub choices: Vec<ConfigChoice>, pub toggle_label: String, pub checked: bool,
    pub reset_label: String, pub save_label: String,
}
impl Render for ConfigEditor {
    fn render(&self) -> Markup { html! {
        label class="ui-input-label" { (&self.name_label) input class="ui-input" value=(&self.name); }
        label class="ui-select-label" { (&self.selection_label) select class="ui-select" { @for choice in &self.choices { option { (&choice.label) } } } }
        label class="ui-toggle" { span class="ui-toggle__switch" { input class="ui-toggle__input" type="checkbox" checked[self.checked]; span class="ui-toggle__slider" {} } span class="ui-toggle__label" { (&self.toggle_label) } }
        div class="test-config-actions" { button type="reset" class="ui-button ui-button--secondary" { (&self.reset_label) } button type="button" class="ui-button ui-button--primary" data-test-config-save { (&self.save_label) } }
    } }
}

/// A four-column settings row.
#[derive(Clone, Debug)]
pub struct SettingsRow { pub setting: String, pub value: String, pub source: String, pub status: String }
/// A table of settings with caller-supplied headings and typed rows.
#[derive(Clone, Debug)]
pub struct SettingsTable { pub headings: [String; 4], pub rows: Vec<SettingsRow> }
impl Render for SettingsTable {
    fn render(&self) -> Markup { html! { div class="ui-table-container" { table class="ui-table" {
        thead { tr { @for heading in &self.headings { th { (heading) } } } }
        tbody { @for row in &self.rows { tr { td { (&row.setting) } td { (&row.value) } td { (&row.source) } td { (&row.status) } } } }
    } } } }
}

/// Selects the enclosing card layout without exposing a class name.
#[derive(Clone, Copy, Debug)]
pub enum ViewLayoutKind { Configuration, Health, Animation }
/// A container arranging the cards belonging to a view.
#[derive(Clone)]
pub struct ViewLayout { pub kind: ViewLayoutKind, pub content: Markup }
impl Render for ViewLayout {
    fn render(&self) -> Markup {
        let class = match self.kind { ViewLayoutKind::Configuration => "test-config-layout", ViewLayoutKind::Health => "test-health-layout", ViewLayoutKind::Animation => "animation-grid" };
        html! { div class=(class) { (self.content.clone()) } }
    }
}

/// One labelled health dependency and its current state.
#[derive(Clone, Debug)]
pub struct HealthDependency { pub label: String, pub value: String, pub state: HealthState }
/// A score and dependency list.
#[derive(Clone, Debug)]
pub struct HealthSummary { pub score: String, pub caption: String, pub dependencies: Vec<HealthDependency> }
impl Render for HealthSummary {
    fn render(&self) -> Markup { html! { div class="health-score" { strong { (&self.score) } span { (&self.caption) } }
        ul class="health-dependencies" { @for dep in &self.dependencies { li { (HealthDot(dep.state)) (&dep.label) " " strong { (&dep.value) } } } }
    } }
}

/// An item in a diagnostic timeline.
#[derive(Clone, Debug)]
pub struct TimelineEntry { pub time: String, pub outcome: String, pub description: String }
/// An ordered diagnostic timeline.
#[derive(Clone, Debug)]
pub struct HealthTimeline { pub entries: Vec<TimelineEntry> }
impl Render for HealthTimeline {
    fn render(&self) -> Markup { html! { ol class="health-timeline" { @for entry in &self.entries { li { time { (&entry.time) } span { (&entry.outcome) } span { (&entry.description) } } } } } }
}
/// One labelled diagnostic count.
#[derive(Clone, Debug)]
pub struct ResultCount { pub value: String, pub label: String }
/// A group of diagnostic counts.
#[derive(Clone, Debug)]
pub struct HealthResults { pub counts: Vec<ResultCount> }
impl Render for HealthResults {
    fn render(&self) -> Markup { html! { div class="health-results" { @for count in &self.counts { div { strong { (&count.value) } span { (&count.label) } } } } } }
}
/// A diagnostic readout escaped as text rather than interpreted as markup.
#[derive(Clone, Debug)]
pub struct HealthOutput { pub text: String }
impl Render for HealthOutput {
    fn render(&self) -> Markup {
        // This is text content, not an attribute: quotes need not be escaped.
        let escaped = self.text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
        html! { pre class="readout health-json" data-test-health-output { (maud::PreEscaped(escaped)) } }
    }
}
/// A delegated health-check action.
#[derive(Clone, Debug)]
pub struct HealthCheck { pub label: String }
impl Render for HealthCheck {
    fn render(&self) -> Markup { html! { button class="ui-button ui-button--primary" data-test-health-check { (&self.label) } } }
}
