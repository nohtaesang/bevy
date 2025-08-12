//! Enemy (AI-controlled) unit systems

pub mod components;
pub mod spawn_system;
pub mod position_sync;
pub mod ai_movement;
pub mod health_display;
pub mod plugin;

pub use components::Enemy;
pub use spawn_system::spawn_enemies;
pub use position_sync::sync_enemy_positions;
pub use ai_movement::move_enemies_left;
pub use health_display::{spawn_enemy_health_displays, update_enemy_health_displays, cleanup_enemy_health_displays};
pub use plugin::EnemyPlugin;