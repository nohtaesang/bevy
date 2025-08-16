//! Plugin for temporary test systems

use bevy::prelude::*;
use crate::states::AppState;
use super::turn_system::handle_turn_switch;

pub struct TempPlugin;

impl Plugin for TempPlugin {
    fn build(&self, app: &mut App) {
        app
            // Turn switching system
            .add_systems(Update, 
                handle_turn_switch.run_if(in_state(AppState::InGame))
            );
    }
}