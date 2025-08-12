//! Attack overlay system and plugin
//!
//! This module handles attack overlay display and plugin integration

pub mod overlay;
pub mod system;
pub mod plugin;
pub mod range_calculation;
pub mod state;

pub use overlay::*;
pub use system::*;
pub use plugin::*;
pub use range_calculation::find_attackable_tiles;
pub use state::AttackValidation;