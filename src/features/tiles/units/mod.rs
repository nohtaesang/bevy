//! Units system module
//!
//! This module contains unit functionality organized into:
//! - components: Ally, Enemy, Health, AP and related unit components
//! - resources: Unit configurations and resources
//! - systems: Spawn, movement, AI, and synchronization systems
//! - plugin: Plugin for unit functionality

pub mod components;
pub mod resources;
pub mod systems;
pub mod plugin;

pub use components::*;
pub use resources::*;
pub use systems::*;
pub use plugin::UnitsPlugin;