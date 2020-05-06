#[macro_use]
pub mod color;
pub mod ordered_map;
pub mod ssh_config;
pub mod tui;

pub use ordered_map::OrderedMap;
pub use tui::TUI as App;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
