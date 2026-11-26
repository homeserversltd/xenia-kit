//! Inline results, reviews and keyboard hints with delegated seats.
use crate::{html, Markup, Render, fragment::{seat, DataSeat}};

#[derive(Clone, Copy)]
pub enum ResultMark { Done, Refused, Unknown }
impl ResultMark {
    fn parts(self) -> (&'static str, &'static str) {
        match self { Self::Done => ("done", "Done"), Self::Refused => ("refused", "Refused"), Self::Unknown => ("unknown", "Unknown") }
    }
}

#[derive(Clone, Default)]
pub struct ResultLineSeats { pub line: Vec<DataSeat>, pub words: Vec<DataSeat>, pub button: Vec<DataSeat> }
#[derive(Clone, Default)]
pub struct ResultLine { pub mark: Option<ResultMark>, pub words: String, pub button: Option<String>, pub seats: ResultLineSeats }
impl Render for ResultLine {
    fn render(&self) -> Markup {
        seat(html! { p class="ui-result-line" role="status" aria-live="polite" {
            @if let Some(mark) = self.mark { @let (name, word) = mark.parts();
                span class="ui-result-line__mark" data-mark=(name) { span class="ui-result-line__dot" aria-hidden="true" {} (word) }
            }
            (seat(html! { span class="ui-result-line__words" { (&self.words) } }, &self.seats.words))
            @if self.button.is_some() || !self.seats.button.is_empty() {
                (seat(html! { button type="button" class="ui-button ui-button--secondary ui-button--small" { (self.button.as_deref().unwrap_or("")) } }, &self.seats.button))
            }
        } }, &self.seats.line)
    }
}

#[derive(Clone, Default)]
pub struct ReviewTarget { pub name: String, pub note: Option<String>, pub seats: Vec<DataSeat> }
#[derive(Clone, Default)]
pub struct InlineReviewSeats { pub review: Vec<DataSeat>, pub confirm: Vec<DataSeat>, pub cancel: Vec<DataSeat> }
#[derive(Clone, Default)]
pub struct InlineReview {
    pub question: String, pub targets: Vec<ReviewTarget>, pub confirm: String, pub confirm_key: String,
    pub cancel: String, pub cancel_key: String, pub seats: InlineReviewSeats,
}
impl Render for InlineReview {
    fn render(&self) -> Markup {
        seat(html! { section class="ui-inline-review" aria-label=(&self.question) {
            p class="ui-inline-review__question" { (&self.question) }
            ul class="ui-inline-review__targets" {
                @for target in &self.targets {
                    (seat(html! { li { span { (&target.name) } @if let Some(note) = &target.note { span class="ui-inline-review__note" { (note) } } } }, &target.seats))
                }
            }
            div class="ui-inline-review__actions" {
                (seat(html! { button type="button" class="ui-button ui-button--danger ui-button--small" aria-label=(&self.confirm) { kbd aria-hidden="true" { (&self.confirm_key) } (&self.confirm) } }, &self.seats.confirm))
                (seat(html! { button type="button" class="ui-button ui-button--secondary ui-button--small" aria-label=(&self.cancel) { kbd aria-hidden="true" { (&self.cancel_key) } (&self.cancel) } }, &self.seats.cancel))
            }
        } }, &self.seats.review)
    }
}

#[derive(Clone, Default)]
pub struct KeyHint { pub key: String, pub word: String, pub button: bool, pub seats: Vec<DataSeat> }
#[derive(Clone, Default)]
pub struct KeyHints { pub label: String, pub rows: Vec<Vec<KeyHint>> }
impl Render for KeyHints {
    fn render(&self) -> Markup {
        html! { div class="ui-key-hints" aria-label=(&self.label) {
            @for row in &self.rows { div class="ui-key-hints__row" {
                @for hint in row {
                    @if hint.button {
                        (seat(html! { button type="button" class="ui-button ui-button--secondary ui-button--small" aria-label=(&hint.word) { kbd aria-hidden="true" { (&hint.key) } (&hint.word) } }, &hint.seats))
                    } @else { span class="ui-key-hints__hint" { kbd { (&hint.key) } (&hint.word) } }
                }
            } }
        } }
    }
}
