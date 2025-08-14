//! Tile system
//!
//! This module contains tile-related components and systems organized in a hybrid structure.

pub mod core;
pub mod selection;
pub mod interaction;
pub mod command;
pub mod visual;
pub mod units;
pub mod plugin;

// Re-export core components
pub use core::{Tile, TileConfig, TileMap, TileContent};
pub use core::{tile_to_world_coords, world_to_tile_coords};

// Re-export selection resources
pub use selection::SelectionCtx;

// Re-export selection systems
pub use selection::{
    handle_idle_state_click,
    handle_tile_selected_click, 
    handle_unit_selected_click,
    handle_enemy_selected_click,
    handle_unit_keyboard_input,
    select_tile, select_unit, select_enemy, clear_selection,
};

// Re-export interaction systems
pub use interaction::{
    handle_right_click_action_cancel,
    handle_right_click_selection_clear,
};

// Re-export command systems
pub use command::{
    handle_move_state_click,
    handle_attack_state_click,
    process_command_queue,
    CommandQueue,
    PendingCommand,
    CommandResult,
};

// Re-export visual systems
pub use visual::{
    MovementValidation,
    AttackValidation,
};

// Re-export unit components
pub use units::{Unit, Enemy, AttackDirection, AttackType, AttackRange, UnitsPlugin};

// Re-export main plugin
pub use plugin::TilesPlugin;