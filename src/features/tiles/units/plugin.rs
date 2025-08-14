//! Main plugin for all unit systems

use bevy::prelude::*;
use crate::states::{AppState, in_game::TurnState};
use super::systems::*;

/// Plugin that combines all unit-related systems
pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Note: Unit spawning is now handled by scenario plugins
            
            // Turn-based systems
            .add_systems(Update, move_enemies_left.run_if(in_state(TurnState::EnemyTurn)));
    }
}