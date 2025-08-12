//! Attack plugin

use bevy::prelude::*;
use crate::{
    states::in_game::{UnitCommandState, SelectionState, TurnState},
    features::tiles::handlers::unit_command_handlers::handle_attack_state_click,
};
use super::{cleanup_attack_overlays, attack_overlay_system, update_attack_validation_on_enter};

/// Attack plugin that consolidates attack-related systems
pub struct AttackOverlayPlugin;

impl Plugin for AttackOverlayPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize attack validation resource
            .init_resource::<super::AttackValidation>()
            
            // Update AttackValidation when entering Attack mode
            .add_systems(OnEnter(UnitCommandState::Attack), update_attack_validation_on_enter)
            
            // Cleanup attack overlays when exiting Attack state
            .add_systems(OnExit(UnitCommandState::Attack), cleanup_attack_overlays)
            
            // Attack systems that run during Attack action state
            .add_systems(Update, (
                // Click handling for Attack state
                handle_attack_state_click.run_if(
                    in_state(TurnState::PlayerTurn)
                    .and(in_state(UnitCommandState::Attack))
                ),
                
                // Attack overlay display
                attack_overlay_system.run_if(
                    in_state(TurnState::PlayerTurn)
                    .and(in_state(UnitCommandState::Attack))
                    .and(in_state(SelectionState::UnitSelected))
                ),
            ));
    }
}