//! Ally (player-controlled) unit systems

pub mod components;
pub mod spawn_system;
pub mod position_sync;
pub mod refresh_system;
pub mod health_display;
pub mod plugin;

pub use components::Unit;
pub use spawn_system::spawn_units;
pub use position_sync::sync_unit_positions;
pub use refresh_system::refresh_units_on_player_turn;
pub use health_display::{spawn_ally_health_displays, update_ally_health_displays, cleanup_ally_health_displays};
pub use plugin::AllyPlugin;