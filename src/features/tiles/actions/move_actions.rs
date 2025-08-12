//! Movement action functions
//!
//! Reusable functions for executing movement

use bevy::prelude::*;
use crate::{
    core::{ActionState, SelectionCtx},
    features::{
        units::Unit,
        tiles::tile_to_world_coords,
    },
    resources::{TileConfig, TileMap},
};

/// Low-level movement execution function
pub fn execute_movement(
    unit: &mut Unit,
    target_pos: IVec2,
    next_action_state: &mut ResMut<NextState<ActionState>>,
) -> bool {
    // Calculate Manhattan distance for movement validation
    let distance = (unit.tile_pos.x - target_pos.x).abs() + (unit.tile_pos.y - target_pos.y).abs();
    
    if distance <= unit.movement_range {
        // Execute movement
        unit.tile_pos = target_pos;
        unit.movement_range -= distance;
        
        // Return to idle action state after movement
        next_action_state.set(ActionState::Idle);
        
        println!("Unit moved to ({}, {}), remaining movement: {}", 
                 target_pos.x, target_pos.y, unit.movement_range);
        
        true
    } else {
        println!("Movement failed: distance {} exceeds remaining movement range {}", 
                 distance, unit.movement_range);
        false
    }
}

/// Execute movement to target position with Transform update
pub fn execute_move(
    target_pos: IVec2,
    next_action_state: &mut ResMut<NextState<ActionState>>,
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
            // Calculate Manhattan distance for movement validation
            let distance = (unit.tile_pos.x - target_pos.x).abs() + (unit.tile_pos.y - target_pos.y).abs();
            
            if distance <= unit.movement_range {
                let old_pos = unit.tile_pos;
                
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
                next_action_state.set(ActionState::Idle);
                
                println!("Unit moved to ({}, {}), remaining movement: {}", 
                         target_pos.x, target_pos.y, unit.movement_range);
                
                return true;
            } else {
                println!("Movement failed: distance {} exceeds remaining movement range {}", 
                         distance, unit.movement_range);
                return false;
            }
        }
    }
    println!("Movement failed - no unit selected");
    false
}