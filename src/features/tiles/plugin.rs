//! Main plugin for tile systems

use bevy::{prelude::*, ecs::schedule::common_conditions::on_event};
use crate::states::{AppState, in_game::{TurnState, SelectionState, UnitCommandState}};
use super::{
    core::{CoreTilesPlugin, TilesSet},
    selection::{
        SelectionCtx, 
        handle_click_idle, handle_click_unit_selected, 
        handle_click_enemy_selected, handle_click_tile_selected
    },
    interaction::{InteractionPlugin, ClickTargetEvent},
    visual::VisualPlugin,
    command::*,
    units::UnitsPlugin,
};

/// Plugin that combines all tile-related systems
pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add core tiles plugin (handles GridIndex, events, basic resources)
            .add_plugins(CoreTilesPlugin)
            
            // Add sub-plugins in order: interaction → visual
            .add_plugins((
                InteractionPlugin,  // Handles input and updates HoverTile
                VisualPlugin,       // Reads HoverTile and renders
                UnitsPlugin,
            ))
            
            // Initialize selection resources
            .init_resource::<SelectionCtx>()
            
            // Initialize command system resources
            .init_resource::<CommandQueue>()
            .init_resource::<CommandConfig>()
            .insert_resource(PathCache::new(1000))
            .init_resource::<CommandStats>()
            .init_resource::<CommandSystemState>()
            
            // Initialize visual resources
            .init_resource::<crate::features::tiles::visual::MovementValidation>()
            .init_resource::<crate::features::tiles::visual::AttackValidation>()
            
            // Add command events
            .add_event::<CommandCompletedEvent>()
            
            // Initialize sub-states
            .add_sub_state::<SelectionState>()
            .add_sub_state::<UnitCommandState>()
            
            // Selection click handlers - event-driven (PreUpdate)
            .add_systems(PreUpdate, (
                handle_click_idle
                    .run_if(on_event::<ClickTargetEvent>)
                    .run_if(in_state(TurnState::PlayerTurn))
                    .run_if(in_state(SelectionState::Idle))
                    .run_if(in_state(UnitCommandState::Idle)),
                handle_click_unit_selected
                    .run_if(on_event::<ClickTargetEvent>)
                    .run_if(in_state(TurnState::PlayerTurn))
                    .run_if(in_state(SelectionState::UnitSelected))
                    .run_if(in_state(UnitCommandState::Idle)),
                handle_click_enemy_selected
                    .run_if(on_event::<ClickTargetEvent>)
                    .run_if(in_state(TurnState::PlayerTurn))
                    .run_if(in_state(SelectionState::EnemySelected)),
                handle_click_tile_selected
                    .run_if(on_event::<ClickTargetEvent>)
                    .run_if(in_state(TurnState::PlayerTurn))
                    .run_if(in_state(SelectionState::TileSelected)),
            ))
            
            // Command processing systems (ApplyCommands - 원본 수정)
            .add_systems(Update, (
                process_command_queue,
                cleanup_timed_out_commands,
                handle_command_events,
                debug_command_queue,
            ).in_set(TilesSet::ApplyCommands).run_if(in_state(AppState::InGame)))
            
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