//! Movement overlay system and plugin
//!
//! This module handles movement overlay display and plugin integration

pub mod overlay;
pub mod system;
pub mod plugin;
pub mod pathfinding;
pub mod state;

pub use overlay::*;
pub use system::*;
pub use plugin::*;
pub use pathfinding::find_reachable_tiles;
pub use state::MovementValidation;