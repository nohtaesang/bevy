//! Refresh system for ally units at turn start

use bevy::prelude::*;
use super::Unit;

/// System that resets all units' movement and attack counts when player turn starts
pub fn refresh_units_on_player_turn(
    mut unit_query: Query<(Entity, &mut Unit), With<Unit>>,
) {
    let unit_count = unit_query.iter().count();
    
    for (entity, mut unit) in unit_query.iter_mut() {
        let old_attack_count = unit.attack_count;
        let old_movement_range = unit.movement_range;
        
        unit.reset_turn_actions();
        
    }
}