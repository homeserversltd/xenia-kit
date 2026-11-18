//! Whole-page composites built from the kit's sections.
use crate::{html, Markup, Render, fragment::{seat, DataSeat}, layout::{AddPortalCard, PortalGrid}};

/// A Portals page: tiles, additional cards, then an optional add card.
#[derive(Clone)]
pub struct PortalsPage {
    pub tiles: Markup,
    pub extra_cards: Markup,
    pub add_portal: Option<AddPortalCard>,
    pub frame_seats: Vec<DataSeat>,
    pub grid_seats: Vec<DataSeat>,
}
impl Render for PortalsPage {
    fn render(&self) -> Markup {
        let grid = PortalGrid {
            content: html! { (self.tiles.clone()) (self.extra_cards.clone()) @if let Some(add) = &self.add_portal { (add) } },
            seats: self.grid_seats.clone(),
        };
        seat(html! { div class="portals-tablet" { (grid) } }, &self.frame_seats)
    }
}
