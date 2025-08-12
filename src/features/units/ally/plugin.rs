//! Plugin for ally (player-controlled) units

use bevy::prelude::*;
use crate::states::{AppState, in_game::TurnState};
use super::{spawn_units, sync_unit_positions, refresh_units_on_player_turn, spawn_ally_health_displays, update_ally_health_displays, cleanup_ally_health_displays};

pub struct AllyPlugin;

impl Plugin for AllyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Spawn units when entering InGame state
            .add_systems(OnEnter(AppState::InGame), spawn_units)
            
            // Sync positions whenever units change
            .add_systems(Update, 
                sync_unit_positions.run_if(in_state(AppState::InGame))
            )
            
            // Refresh units when player turn starts
            .add_systems(OnEnter(TurnState::PlayerTurn), refresh_units_on_player_turn)
            
            // Health display systems
            .add_systems(Update, (
                spawn_ally_health_displays,
                update_ally_health_displays,
                cleanup_ally_health_displays,
            ).run_if(in_state(AppState::InGame)));
    }
}