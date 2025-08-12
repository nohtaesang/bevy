//! Selection state-specific click handlers
//!
//! This module contains handlers that route clicks based on SelectionState

pub mod idle_handler;
pub mod tile_selected_handler;
pub mod unit_selected_handler;
pub mod enemy_selected_handler;
pub mod unit_keyboard_input;

pub use idle_handler::handle_idle_state_click;
pub use tile_selected_handler::handle_tile_selected_click;
pub use unit_selected_handler::handle_unit_selected_click;
pub use enemy_selected_handler::handle_enemy_selected_click;
pub use unit_keyboard_input::handle_unit_keyboard_input;