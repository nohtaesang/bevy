//! Main plugin for tile systems

use bevy::prelude::*;
use crate::states::{AppState, in_game::{TurnState, SelectionState, UnitCommandState}};
use super::{
    resources::{TileConfig, TileMap, SelectionCtx},
    spawn_system::spawn_tiles,
    handlers::{
        selection_handlers::*,
        unit_command_handlers::*,
        right_click::*,
    },
    overlay::{
        hover::HoverOverlayPlugin,
        selected::SelectedOverlayPlugin,
        movement::MovementOverlayPlugin,
        attack::AttackOverlayPlugin,
    },
};

/// Plugin that combines all tile-related systems
pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<TileConfig>()
            .init_resource::<SelectionCtx>()
            .insert_resource(TileMap::new(30)) // Initialize with 30x30 grid
            
            // Initialize sub-states
            .add_sub_state::<SelectionState>()
            .add_sub_state::<UnitCommandState>()
            
            // Add overlay plugins
            .add_plugins((
                HoverOverlayPlugin,
                SelectedOverlayPlugin,
                MovementOverlayPlugin,
                AttackOverlayPlugin,
            ))
            
            // Spawn tiles when entering InGame
            .add_systems(OnEnter(AppState::InGame), spawn_tiles)
            
            // State-specific click handlers
            .add_systems(Update, (
                handle_idle_state_click.run_if(
                    in_state(TurnState::PlayerTurn)
                    .and(in_state(SelectionState::Idle))
                ),
                handle_tile_selected_click.run_if(
                    in_state(TurnState::PlayerTurn)
                    .and(in_state(SelectionState::TileSelected))
                ),
                handle_unit_selected_click.run_if(
                    in_state(TurnState::PlayerTurn)
                    .and(in_state(SelectionState::UnitSelected))
                ),
                handle_enemy_selected_click.run_if(
                    in_state(TurnState::PlayerTurn)
                    .and(in_state(SelectionState::EnemySelected))
                ),
            ))
            
            // Keyboard input handlers
            .add_systems(Update, (
                handle_unit_keyboard_input.run_if(
                    in_state(TurnState::PlayerTurn)
                    .and(in_state(SelectionState::UnitSelected))
                ),
            ))
            
            // Right-click handlers  
            .add_systems(Update, (
                handle_right_click_action_cancel.run_if(in_state(TurnState::PlayerTurn)),
                handle_right_click_selection_clear.run_if(in_state(TurnState::PlayerTurn)),
            ))
            
            // Clear selection when PlayerTurn ends
            .add_systems(OnExit(TurnState::PlayerTurn), clear_selection_on_turn_end);
    }
}

/// Clear selection information when PlayerTurn ends
fn clear_selection_on_turn_end(
    mut selection_ctx: ResMut<SelectionCtx>,
    mut selection_state: ResMut<NextState<SelectionState>>,
    mut unit_command_state: ResMut<NextState<UnitCommandState>>,
) {
    // Clear selection context
    *selection_ctx = SelectionCtx::default();
    
    // Reset states to default
    selection_state.set(SelectionState::Idle);
    unit_command_state.set(UnitCommandState::Idle);
}