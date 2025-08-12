//! Refresh system for ally units at turn start

use bevy::prelude::*;
use super::Unit;

/// System that resets all units' movement and attack counts when player turn starts
pub fn refresh_units_on_player_turn(
    mut unit_query: Query<&mut Unit, With<Unit>>,
) {
    for mut unit in unit_query.iter_mut() {
        unit.reset_turn_actions();
        info!("Unit refreshed: movement_range={}, attack_count={}", 
              unit.movement_range, unit.attack_count);
    }
    info!("All units refreshed for new player turn");
}