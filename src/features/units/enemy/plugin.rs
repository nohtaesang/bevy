//! Plugin for enemy (AI-controlled) units

use bevy::prelude::*;
use crate::states::{AppState, in_game::TurnState};
use super::{spawn_enemies, sync_enemy_positions, move_enemies_left, spawn_enemy_health_displays, update_enemy_health_displays, cleanup_enemy_health_displays};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Spawn enemies when entering InGame state
            .add_systems(OnEnter(AppState::InGame), spawn_enemies)
            
            // Sync positions whenever enemies change
            .add_systems(Update, 
                sync_enemy_positions.run_if(in_state(AppState::InGame))
            )
            
            // Move enemies when enemy turn starts
            .add_systems(OnEnter(TurnState::EnemyTurn), move_enemies_left)
            
            // Health display systems
            .add_systems(Update, (
                spawn_enemy_health_displays,
                update_enemy_health_displays,
                cleanup_enemy_health_displays,
            ).run_if(in_state(AppState::InGame)));
    }
}