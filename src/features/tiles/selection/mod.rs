//! Selection system module
//!
//! This module contains all selection-related functionality organized into:
//! - components: Selection-related component definitions
//! - resources: Selection context and state resources  
//! - systems: Input handling, state transitions, and visual effects

pub mod components;
pub mod resources; 
pub mod systems;

pub use components::*;
pub use resources::*;
pub use systems::*;