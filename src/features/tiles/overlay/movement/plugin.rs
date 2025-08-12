//! Movement plugin

use bevy::prelude::*;
use crate::{
    states::in_game::{UnitCommandState, SelectionState, TurnState},
    features::tiles::handlers::unit_command_handlers::handle_move_state_click,
};
use super::{cleanup_movement_overlays, movement_overlay_system};

/// Movement plugin that consolidates movement-related systems
pub struct MovementOverlayPlugin;

impl Plugin for MovementOverlayPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize movement validation resource
            .init_resource::<super::MovementValidation>()
            
            // Cleanup movement overlays when exiting Move state
            .add_systems(OnExit(UnitCommandState::Move), cleanup_movement_overlays)
            
            // Movement systems that run during Move action state
            .add_systems(Update, (
                // Click handling for Move state
                handle_move_state_click.run_if(
                    in_state(TurnState::PlayerTurn)
                    .and(in_state(UnitCommandState::Move))
                ),
                
                // Movement overlay display
                movement_overlay_system.run_if(
                    in_state(TurnState::PlayerTurn)
                    .and(in_state(UnitCommandState::Move))
                    .and(in_state(SelectionState::UnitSelected))
                ),
            ));
    }
}