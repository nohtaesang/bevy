//! Tile system
//!
//! This module contains tile-related components and systems.

pub mod tiles;
pub mod utils;
pub mod spawn_system;
pub mod overlay;
pub mod actions;
pub mod handlers;

pub use tiles::Tile;
pub use utils::{tile_to_world_coords, find_reachable_tiles};
pub use spawn_system::spawn_tiles;
pub use overlay::{
    HoverPlugin,
    SelectedPlugin,
    MovementPlugin,
};
pub use handlers::{
    handle_idle_state_click,
    handle_move_state_click,
    handle_attack_state_click,
    handle_tile_selected_click,
    handle_unit_selected_click,
    handle_enemy_selected_click,
    handle_unit_keyboard_input,
    handle_right_click_action_cancel,
    handle_right_click_selection_clear,
};