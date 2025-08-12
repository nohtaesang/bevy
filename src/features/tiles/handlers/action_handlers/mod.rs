//! Action state-specific click handlers
//!
//! This module contains handlers that route clicks based on ActionState

pub mod move_handler;
pub mod attack_handler;

pub use move_handler::handle_move_state_click;
pub use attack_handler::handle_attack_state_click;