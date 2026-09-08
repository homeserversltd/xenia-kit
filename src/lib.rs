//! Household cartridges compose the crown's markup and load public seats at startup.
pub mod events;
pub mod fragment;
pub mod health;
pub mod manifest;
pub mod theme;
pub mod transport;
pub mod ux;
pub use fragment::Fragment;
pub use manifest::{Manifest, Schemas};
pub use maud::{html, Markup, Render};
