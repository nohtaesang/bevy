//! UI systems and components
//!
//! This module contains all user interface related functionality.

pub mod state_display;
pub mod hover_info;
pub mod unit_info;
pub mod plugin;

pub use state_display::{setup_state_display_ui, update_state_display};
pub use hover_info::{setup_hover_info_ui, update_hover_info, cleanup_hover_info_ui};
pub use unit_info::{setup_unit_info_ui, update_unit_info, cleanup_unit_info_ui};
pub use plugin::UIPlugin;