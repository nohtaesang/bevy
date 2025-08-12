//! UI systems and components
//!
//! This module contains all user interface related functionality.

pub mod state_display;
pub mod plugin;

pub use state_display::{setup_state_display_ui, update_state_display};
pub use plugin::UIPlugin;