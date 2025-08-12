//! Attack plugin

use bevy::prelude::*;
use crate::{
    core::{ActionState, SelectionState, TurnState},
    features::tiles::handlers::action_handlers::handle_attack_state_click,
};
use super::{cleanup_attack_overlays, attack_overlay_system};

/// Attack plugin that consolidates attack-related systems
pub struct AttackOverlayPlugin;

impl Plugin for AttackOverlayPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize attack validation resource
            .init_resource::<super::AttackValidation>()
            
            // Cleanup attack overlays when exiting Attack state
            .add_systems(OnExit(ActionState::Attack), cleanup_attack_overlays)
            
            // Attack systems that run during Attack action state
            .add_systems(Update, (
                // Click handling for Attack state
                handle_attack_state_click.run_if(
                    in_state(TurnState::PlayerTurn)
                    .and(in_state(ActionState::Attack))
                ),
                
                // Attack overlay display
                attack_overlay_system.run_if(
                    in_state(TurnState::PlayerTurn)
                    .and(in_state(ActionState::Attack))
                    .and(in_state(SelectionState::UnitSelected))
                ),
            ));
    }
}