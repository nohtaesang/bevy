//! Interaction system module
//!
//! This module handles raw input collection and common interaction context
//! without domain-specific knowledge

pub mod resources;
pub mod events;
pub mod systems;
pub mod hotkeys;
pub mod plugin;

pub use resources::*;
pub use events::*;
pub use systems::*;
pub use hotkeys::*;
pub use plugin::InteractionPlugin;

/// Prelude for visual systems to import interaction types
pub mod prelude {
    pub use super::resources::HoverTile;
}