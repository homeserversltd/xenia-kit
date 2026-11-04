//! Small semantic presentation parts for composing labelled specimens.
use crate::{html, Markup, Render};

/// A semantic text container; children are already-escaped markup.
#[derive(Clone, Copy, Debug)]
pub enum TextKind { Paragraph, MutedParagraph, Strong, Code, Span, Div, Heading3, Heading4, Heading5 }
/// Wraps composed text or inline components in a semantic element.
#[derive(Clone)]
pub struct Text { pub kind: TextKind, pub content: Markup }
impl Render for Text {
    fn render(&self) -> Markup { let c=self.content.clone(); match self.kind {
        TextKind::Paragraph => html!{p{(c)}}, TextKind::MutedParagraph => html!{p class="muted"{(c)}},
        TextKind::Strong => html!{strong{(c)}}, TextKind::Code => html!{code{(c)}},
        TextKind::Span => html!{span{(c)}}, TextKind::Div => html!{div{(c)}}, TextKind::Heading3 => html!{h3{(c)}},
        TextKind::Heading4 => html!{h4{(c)}}, TextKind::Heading5 => html!{h5{(c)}},
    }}
}

/// A paragraph from text without treating caller content as HTML.
#[derive(Clone, Debug)]
pub struct Paragraph { pub text: String }
impl Render for Paragraph { fn render(&self) -> Markup { html!{p{(&self.text)}} } }

/// A layout for a row of controls.
#[derive(Clone)]
pub struct ShowcaseRow { pub content: Markup, pub modal_controls: bool }
impl Render for ShowcaseRow {
    fn render(&self) -> Markup { html!{div class="showcase-row" data-modal-catalog=[self.modal_controls.then_some("working-controls")]{(self.content.clone())}} }
}

/// An unlabelled showcase item; headings can precede it independently.
#[derive(Clone)]
pub struct ShowcaseBox { pub content: Markup }
impl Render for ShowcaseBox { fn render(&self) -> Markup {html!{div class="showcase-item"{(self.content.clone())}}} }

/// A labelled article specimen, distinct from a generic div item.
#[derive(Clone)]
pub struct ShowcaseArticle { pub title: String, pub content: Markup }
impl Render for ShowcaseArticle {
    fn render(&self) -> Markup { html!{article class="showcase-item" {h4{(&self.title)} (self.content.clone())}} }
}

/// A list of markup entries, rendered as separate ordered or unordered items.
#[derive(Clone)]
pub struct List { pub ordered: bool, pub entries: Vec<Markup> }
impl Render for List {
    fn render(&self) -> Markup { if self.ordered {html!{ol{@for e in &self.entries{li{(e.clone())}}}}} else {html!{ul{@for e in &self.entries{li{(e.clone())}}}}} }
}

/// Optional catalog marker on a showcase grid.
#[derive(Clone, Copy, Debug)]
pub enum CatalogKind { Visibility, Spinner, Toast, UploadLibrary }
/// A grid with a known catalog's delegated markers.
#[derive(Clone)]
pub struct CatalogGrid { pub kind: CatalogKind, pub content: Markup }
impl Render for CatalogGrid {
    fn render(&self) -> Markup { let c=self.content.clone(); match self.kind {
        CatalogKind::Visibility=>html!{div class="showcase-grid visibility-system-catalog" data-visibility-system-catalog{(c)}},
        CatalogKind::Spinner=>html!{div class="showcase-grid" data-loading-spinner-catalog data-catalog-specimen="loading-spinner"{(c)}},
        CatalogKind::Toast=>html!{div class="showcase-grid" data-toast-catalog data-catalog-specimen="toast-notifications"{(c)}},
        CatalogKind::UploadLibrary=>html!{div class="showcase-grid" data-test-upload-library{(c)}},
    }}
}

/// A diagnostic slider value, updated by the host's delegated control.
#[derive(Clone, Debug)]
pub struct SliderValue { pub label: String, pub value: String }
impl Render for SliderValue {
    fn render(&self) -> Markup { html!{p{(&self.label) " " span data-slider-value{(&self.value)}}} }
}
