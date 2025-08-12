//! Movement action functions
//!
//! Reusable functions for executing movement

use bevy::prelude::*;
use crate::{
    states::in_game::UnitCommandState,
    features::{
        tiles::{SelectionCtx, tile_to_world_coords, TileConfig, TileMap, overlay::movement::pathfinding::find_reachable_tiles},
        units::Unit,
    },
};

/// Low-level movement execution function
pub fn execute_movement(
    unit: &mut Unit,
    target_pos: IVec2,
    next_action_state: &mut ResMut<NextState<UnitCommandState>>,
) -> bool {
    // Calculate Manhattan distance for movement validation
    let distance = (unit.tile_pos.x - target_pos.x).abs() + (unit.tile_pos.y - target_pos.y).abs();
    
    if distance <= unit.movement_range {
        // Execute movement
        unit.tile_pos = target_pos;
        unit.movement_range -= distance;
        
        // Return to idle action state after movement
        next_action_state.set(UnitCommandState::Idle);
        
        
        true
    } else {
        false
    }
}

/// Execute movement to target position with Transform update
pub fn execute_move(
    target_pos: IVec2,
    next_action_state: &mut ResMut<NextState<UnitCommandState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
    tile_config: &TileConfig,
    tile_map: &mut ResMut<TileMap>,
    unit_queries: &mut ParamSet<(
        Query<(Entity, &Unit)>,
        Query<(&mut Unit, &mut Transform)>,
    )>,
) -> bool {
    if let Some(selected_unit_entity) = selection_ctx.selected_unit {
        if let Ok((mut unit, mut transform)) = unit_queries.p1().get_mut(selected_unit_entity) {
            // Use pathfinding to validate movement (same as overlay system)
            let reachable_tiles = find_reachable_tiles(
                unit.tile_pos,
                unit.movement_range,
                tile_config,
                tile_map,
            );
            
            // Check if target position is reachable
            if reachable_tiles.contains(&target_pos) {
                let old_pos = unit.tile_pos;
                
                // Calculate actual movement cost using Manhattan distance
                // This should match the pathfinding algorithm's step-by-step cost
                let distance = (unit.tile_pos.x - target_pos.x).abs() + (unit.tile_pos.y - target_pos.y).abs();
                
                // Update unit's logical position
                unit.tile_pos = target_pos;
                unit.movement_range -= distance;
                
                // Update unit's visual position (Transform)
                let world_pos = tile_to_world_coords(target_pos.x, target_pos.y, tile_config);
                transform.translation.x = world_pos.x;
                transform.translation.y = world_pos.y;
                
                // Update TileMap - move the unit from old position to new position
                tile_map.move_unit(old_pos, target_pos);
                
                // Update selection context
                selection_ctx.tile = Some(target_pos);
                
                // Return to idle action state after movement
                next_action_state.set(UnitCommandState::Idle);
                
                
                return true;
            } else {
                return false;
            }
        }
    }
    false
}