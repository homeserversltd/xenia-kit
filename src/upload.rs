//! Reusable upload-domain layouts built from escaped, typed markup inputs.
//!
//! These builders emit markup only. Styling comes from the host's existing
//! classes, while controls remain inert for the host to wire through its own
//! delegated interaction layer.
use crate::{html, Markup, Render};

/// One path segment in the directory location shown above a tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectoryCrumb {
    pub label: String,
    pub path: String,
    pub current: bool,
}

impl DirectoryCrumb {
    pub fn new(label: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            path: path.into(),
            current: false,
        }
    }

    pub fn current(mut self) -> Self {
        self.current = true;
        self
    }
}

/// Breadcrumb navigation and the selected directory readout.
#[derive(Clone, Debug)]
pub struct DirectoryLocation {
    pub label: String,
    pub selected_path: String,
    pub crumbs: Vec<DirectoryCrumb>,
}

impl DirectoryLocation {
    pub fn new(
        label: impl Into<String>,
        selected_path: impl Into<String>,
        crumbs: Vec<DirectoryCrumb>,
    ) -> Self {
        Self {
            label: label.into(),
            selected_path: selected_path.into(),
            crumbs,
        }
    }
}

impl Render for DirectoryLocation {
    fn render(&self) -> Markup {
        html! {
            div class="directory-breadcrumb-container" {
                nav class="breadcrumb-navigation" data-test-domain-breadcrumbs aria-label=(&self.label) {
                    @for (index, crumb) in self.crumbs.iter().enumerate() {
                        @if index > 0 { span class="breadcrumb-separator" { " / " } }
                        button type="button"
                            class=(if crumb.current { "breadcrumb-item current" } else { "breadcrumb-item" })
                            data-test-breadcrumb-path=(&crumb.path)
                            aria-current=[crumb.current.then_some("page")] {
                            (&crumb.label)
                        }
                    }
                }
                p {
                    "Selected directory: "
                    code data-test-domain-path { (&self.selected_path) }
                }
            }
        }
    }
}

/// Expand affordance for one visible row in a directory tree.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DirectoryExpansion {
    #[default]
    Leaf,
    Expanded,
    Collapsed,
}

/// A typed directory row. Depth and expansion state are data, not caller CSS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectoryEntry {
    pub path: String,
    pub name: String,
    pub selected: bool,
    pub depth: u16,
    pub expansion: DirectoryExpansion,
}

impl DirectoryEntry {
    pub fn new(path: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            name: name.into(),
            selected: false,
            depth: 0,
            expansion: DirectoryExpansion::Leaf,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn at_depth(mut self, depth: u16) -> Self {
        self.depth = depth;
        self
    }

    pub fn expansion(mut self, expansion: DirectoryExpansion) -> Self {
        self.expansion = expansion;
        self
    }
}

/// Directory-tree rows for a selected directory browser.
#[derive(Clone, Debug)]
pub struct DirectoryTree {
    pub label: String,
    pub entries: Vec<DirectoryEntry>,
}

impl DirectoryTree {
    pub fn new(label: impl Into<String>, entries: Vec<DirectoryEntry>) -> Self {
        Self {
            label: label.into(),
            entries,
        }
    }
}

impl Render for DirectoryTree {
    fn render(&self) -> Markup {
        html! {
            div class="directory-tree-container" role="tree" aria-label=(&self.label) {
                @for entry in &self.entries {
                    @let selected = entry.selected.to_string();
                    @let expansion = match entry.expansion {
                        DirectoryExpansion::Leaf => None,
                        DirectoryExpansion::Expanded => Some("true"),
                        DirectoryExpansion::Collapsed => Some("false"),
                    };
                    button type="button"
                        class=(if entry.selected { "directory-entry selected" } else { "directory-entry" })
                        role="treeitem"
                        aria-selected=(selected)
                        aria-level=(u32::from(entry.depth) + 1)
                        aria-expanded=[expansion]
                        data-test-directory-path=(&entry.path) {
                        @match entry.expansion {
                            DirectoryExpansion::Expanded => span class="expand-control" { "−" },
                            DirectoryExpansion::Collapsed => span class="expand-control" { "+" },
                            DirectoryExpansion::Leaf => span class="expand-control" {},
                        }
                        i class=(if entry.selected {
                                "fas fa-folder-open entry-icon"
                            } else {
                                "fas fa-folder entry-icon"
                            }) aria-hidden="true" {}
                        span class="entry-name" { (&entry.name) }
                        @if entry.selected { span class="entry-selected" { "selected" } }
                    }
                }
            }
        }
    }
}

/// Directory browser chrome, location, and selectable directory rows.
#[derive(Clone, Debug)]
pub struct DirectoryBrowser {
    pub location: DirectoryLocation,
    pub tree: DirectoryTree,
    pub refresh_label: String,
    pub unavailable_action_label: String,
}

impl DirectoryBrowser {
    pub fn new(
        location: DirectoryLocation,
        tree: DirectoryTree,
        refresh_label: impl Into<String>,
        unavailable_action_label: impl Into<String>,
    ) -> Self {
        Self {
            location,
            tree,
            refresh_label: refresh_label.into(),
            unavailable_action_label: unavailable_action_label.into(),
        }
    }
}

impl Render for DirectoryBrowser {
    fn render(&self) -> Markup {
        html! {
            div class="directory-browser" data-test-directory-browser {
                div class="directory-browser-header" {
                    button type="button" class="refresh-button icon-button" aria-label=(&self.refresh_label) {
                        i class="fas fa-rotate" aria-hidden="true" {}
                    }
                    button type="button" disabled { (&self.unavailable_action_label) }
                }
                (&self.location)
                (&self.tree)
            }
        }
    }
}

/// File-selection controls for an upload destination.
#[derive(Clone, Debug)]
pub struct UploadControls {
    pub input_label: String,
    pub submit_label: String,
    pub selection_summary: String,
    pub multiple: bool,
    pub submit_disabled: bool,
}

impl UploadControls {
    pub fn new(
        input_label: impl Into<String>,
        submit_label: impl Into<String>,
        selection_summary: impl Into<String>,
    ) -> Self {
        Self {
            input_label: input_label.into(),
            submit_label: submit_label.into(),
            selection_summary: selection_summary.into(),
            multiple: true,
            submit_disabled: true,
        }
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn submit_disabled(mut self, disabled: bool) -> Self {
        self.submit_disabled = disabled;
        self
    }
}

impl Render for UploadControls {
    fn render(&self) -> Markup {
        html! {
            div class="file-upload-section" data-test-domain-file-section {
                input type="file" multiple[self.multiple]
                    data-test-domain-file aria-label=(&self.input_label);
                button type="button" data-test-domain-submit disabled[self.submit_disabled] {
                    (&self.submit_label)
                }
                span data-test-domain-file-name { (&self.selection_summary) }
            }
        }
    }
}

/// A bounded percent value; values above 100 are clamped at construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UploadPercent(u8);

impl UploadPercent {
    pub fn new(value: u8) -> Self {
        Self(value.min(100))
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

/// Typed domain status for an upload row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UploadStatus {
    Pending,
    Uploading,
    Completed,
    Error,
}

impl UploadStatus {
    fn class(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Uploading => "uploading",
            Self::Completed => "completed",
            Self::Error => "error",
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Self::Pending => "fa-hourglass-half",
            Self::Uploading => "fa-upload",
            Self::Completed => "fa-check",
            Self::Error => "fa-times",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Uploading => "Uploading",
            Self::Completed => "Completed",
            Self::Error => "Error",
        }
    }

    fn progress_label(self) -> &'static str {
        match self {
            Self::Pending => "Queued upload",
            Self::Uploading => "Uploading file",
            Self::Completed => "Completed upload",
            Self::Error => "Failed upload",
        }
    }
}

/// One accessible upload progress row with optional speed and failure detail.
#[derive(Clone, Debug)]
pub struct UploadProgress {
    pub filename: String,
    pub status: UploadStatus,
    pub percent: UploadPercent,
    pub transferred: String,
    pub total: String,
    pub speed: Option<String>,
    pub error: Option<String>,
}

impl UploadProgress {
    pub fn new(
        filename: impl Into<String>,
        status: UploadStatus,
        percent: u8,
        transferred: impl Into<String>,
        total: impl Into<String>,
    ) -> Self {
        Self {
            filename: filename.into(),
            status,
            percent: UploadPercent::new(percent),
            transferred: transferred.into(),
            total: total.into(),
            speed: None,
            error: None,
        }
    }

    pub fn with_speed(mut self, speed: impl Into<String>) -> Self {
        self.speed = Some(speed.into());
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }
}

impl Render for UploadProgress {
    fn render(&self) -> Markup {
        let percent = self.percent.value();
        let progress_text = format!("{percent}%");
        html! {
            article class=(format!("upload-progress {}", self.status.class())) {
                div class="upload-header" {
                    i class=(format!("fas {} status-icon", self.status.icon())) aria-hidden="true" {}
                    span class="filename" { (&self.filename) }
                }
                div class="progress-section" {
                    div class="progress-bar-container" role="progressbar"
                        aria-label=(self.status.progress_label()) aria-valuemin="0"
                        aria-valuemax="100" aria-valuenow=(percent) {
                        div class="progress-bar" style=(format!("width:{percent}%")) {
                            span class="progress-text" { (&progress_text) }
                        }
                    }
                    div class="upload-stats" {
                        span class="size" {
                            (&self.transferred) " / " (&self.total)
                        }
                        @if let Some(speed) = &self.speed {
                            span class="speed" { (speed) }
                        } @else {
                            span { (self.status.label()) }
                        }
                    }
                    @if let Some(error) = &self.error {
                        div class="error-message" role="alert" { (error) }
                    }
                }
            }
        }
    }
}

/// Identifies the upload list location without exposing markup class names.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UploadProgressPlacement {
    #[default]
    DomainPack,
    ProgressPanel,
}

/// A reusable list of upload status rows.
#[derive(Clone, Debug, Default)]
pub struct UploadProgressList {
    pub rows: Vec<UploadProgress>,
    pub placement: UploadProgressPlacement,
}

impl UploadProgressList {
    pub fn new(rows: Vec<UploadProgress>, placement: UploadProgressPlacement) -> Self {
        Self { rows, placement }
    }
}

impl Render for UploadProgressList {
    fn render(&self) -> Markup {
        let in_domain_pack = self.placement == UploadProgressPlacement::DomainPack;
        let in_progress_panel = self.placement == UploadProgressPlacement::ProgressPanel;
        html! {
            div class="upload-progress-list"
                data-test-upload-progress-list[in_domain_pack]
                data-test-progress-states[in_progress_panel] {
                @for row in &self.rows { (row) }
            }
        }
    }
}

/// Complete upload-domain pack: progress, directory selection, and file controls.
#[derive(Clone, Debug)]
pub struct UploadPack {
    pub progress: UploadProgressList,
    pub browser: DirectoryBrowser,
    pub controls: UploadControls,
}

impl UploadPack {
    pub fn new(
        progress: Vec<UploadProgress>,
        browser: DirectoryBrowser,
        controls: UploadControls,
    ) -> Self {
        Self {
            progress: UploadProgressList::new(
                progress,
                UploadProgressPlacement::DomainPack,
            ),
            browser,
            controls,
        }
    }
}

impl Render for UploadPack {
    fn render(&self) -> Markup {
        html! {
            div class="upload-tablet" data-test-upload-domain-pack data-catalog-specimen="upload-domain-pack" {
                (&self.progress)
                div class="upload-controls" {
                    (&self.browser)
                    (&self.controls)
                }
            }
        }
    }
}
