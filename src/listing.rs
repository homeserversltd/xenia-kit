//! Folder listings and the path line, with seats for the owning face.
use maud::{html, Markup, Render};
use crate::composite::CollapsibleBubble;
use crate::fragment::{seat, DataSeat};
use crate::ux::{Icon, IconButton, Shape, Size};

#[derive(Clone, Copy)]
pub enum ListingKind { Folder, File }

#[derive(Clone, Default)]
pub struct ListingRowSeats {
    pub row: Vec<DataSeat>,
    pub input: Vec<DataSeat>,
    pub name: Vec<DataSeat>,
}

#[derive(Clone)]
pub struct ListingRow {
    pub kind: ListingKind,
    pub name: String,
    pub count: Option<u64>,
    pub checked: bool,
    pub cursor: bool,
    pub input_name: Option<String>,
    pub input_value: Option<String>,
    pub seats: ListingRowSeats,
}

impl Render for ListingRow {
    fn render(&self) -> Markup {
        seat(html! {
            li class=(if self.cursor { "ui-listing-row cursor" } else { "ui-listing-row" }) tabindex="-1" {
                span class="ui-listing-row__cursor" aria-hidden="true" { "›" }
                label class="ui-checkbox" {
                    (seat(html! { input type="checkbox" class="ui-checkbox__input" name=[self.input_name.as_deref()] value=[self.input_value.as_deref()] checked[self.checked] aria-label=(format!("Select {}", self.name)); }, &self.seats.input))
                    span class="ui-checkbox__checkmark" {
                        svg class="ui-checkbox__icon" viewBox="0 0 24 24" aria-hidden="true" { path d="M5 12l5 5 9-9" fill="none" {} }
                    }
                }
                (seat(html! { span class="ui-listing-row__name" { (&self.name) @if matches!(self.kind, ListingKind::Folder) { "/" } } }, &self.seats.name))
                @if matches!(self.kind, ListingKind::File) {
                    @if let Some(count) = self.count.filter(|count| *count > 1) {
                        span class="ui-listing-row__linked" { "LINKED ×" (count) }
                    }
                }
            }
        }, &self.seats.row)
    }
}

#[derive(Clone, Default)]
pub struct ListingRows { pub label: String, pub rows: Vec<ListingRow>, pub seats: Vec<DataSeat> }
impl Render for ListingRows {
    fn render(&self) -> Markup {
        seat(html! { ul class="ui-listing-rows" aria-label=(&self.label) { @for row in &self.rows { (row) } } }, &self.seats)
    }
}

#[derive(Clone, Default)]
pub struct Crumb { pub text: String, pub current: bool, pub seats: Vec<DataSeat> }
#[derive(Clone, Default)]
pub struct PocketItem { pub path: String, pub seats: Vec<DataSeat> }
#[derive(Clone, Default)]
pub struct PathLineSeats {
    pub line: Vec<DataSeat>,
    pub nav: Vec<DataSeat>,
    pub pocket: Vec<DataSeat>,
    pub empty: Vec<DataSeat>,
}
#[derive(Clone, Default)]
pub struct PathLine {
    pub label: String,
    pub trail: Vec<Crumb>,
    pub pocket: Vec<PocketItem>,
    pub drop_key: String,
    pub seats: PathLineSeats,
}
impl Render for PathLine {
    fn render(&self) -> Markup {
        let body = if self.pocket.is_empty() {
            html! { p { "Nothing gathered." } }
        } else {
            html! {
                ul class="ui-path-line__items" {
                    @for item in &self.pocket {
                        li { span { (&item.path) }
                            (seat(IconButton { icon:Icon::Text("×"), size:Size::Small, shape:Shape::Circle, label:format!("Remove {} from the pocket", item.path), disabled:false, ghost:true, danger:false, hx:Default::default() }.render(), &item.seats))
                        }
                    }
                }
                (seat(html! { button type="button" class="ui-button ui-button--secondary ui-button--small" { "Empty pocket" } }, &self.seats.empty))
            }
        };
        seat(html! {
            div class="ui-path-line" {
                (seat(html! { nav class="ui-path-line__trail" aria-label=(&self.label) {
                    @for (index, crumb) in self.trail.iter().enumerate() {
                        @if index > 0 { span class="ui-path-line__sep" aria-hidden="true" { " / " } }
                        @if crumb.current {
                            (seat(html! { span class="ui-path-line__here" { (&crumb.text) } }, &crumb.seats))
                        } @else {
                            (seat(html! { button type="button" class="ui-path-line__crumb" { (&crumb.text) } }, &crumb.seats))
                        }
                    }
                } }, &self.seats.nav))
                (CollapsibleBubble { title:"Pocket".into(), line:self.pocket.len().to_string(), open:false, seats:self.seats.pocket.clone(), body })
                span class="ui-path-line__drop" { "drop here: " kbd { (&self.drop_key) } }
            }
        }, &self.seats.line)
    }
}
