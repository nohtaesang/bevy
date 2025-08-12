//! Click handlers organized by state types
//!
//! This module contains all click handling logic organized into submodules

pub mod unit_command_handlers;
pub mod selection_handlers;
pub mod right_click;

pub use unit_command_handlers::*;
pub use selection_handlers::*;
pub use right_click::*;