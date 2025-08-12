use bevy::prelude::*;
use crate::{
    states::{AppState, in_game::{TurnState, SelectionState, UnitCommandState}}, 
    features::{
        camera::CameraPlugin, 
        tiles::{
            TileConfig, TileMap,
            spawn_tiles, 
            handle_idle_state_click,
            handle_tile_selected_click,
            handle_unit_selected_click,
            handle_enemy_selected_click,
            handle_unit_keyboard_input,
            handle_right_click_action_cancel,
            handle_right_click_selection_clear,
            HoverOverlayPlugin,
            SelectedOverlayPlugin,
            MovementOverlayPlugin,
            AttackOverlayPlugin,
            SelectionCtx,
        },
        units::{spawn_units, spawn_enemies},
    },
    ui::{setup_state_display_ui, update_state_display},
};

/// Plugin that handles all InGame state functionality
pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<TileConfig>()
            .init_resource::<SelectionCtx>()
            .insert_resource(TileMap::new(10)) // Initialize with 10x10 grid
            
            // Initialize sub-states
            .add_sub_state::<TurnState>()
            .add_sub_state::<SelectionState>()
            .add_sub_state::<UnitCommandState>()
            
            // Add plugins
            .add_plugins((CameraPlugin, HoverOverlayPlugin, SelectedOverlayPlugin, MovementOverlayPlugin, AttackOverlayPlugin))
            
            // Systems that run when entering InGame state
            .add_systems(OnEnter(AppState::InGame), (
                spawn_tiles,
                spawn_units,
                spawn_enemies,
                setup_state_display_ui,
            ))
            
            // Systems that run every frame while in InGame state
            .add_systems(Update, (
                // UI updates
                update_state_display,
            ).run_if(in_state(AppState::InGame)))
            
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
            
            // Action-specific click handlers are now handled by plugins
            
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
            ));
            

    }
}

