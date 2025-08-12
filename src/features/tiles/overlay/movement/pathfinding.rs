//! Pathfinding for movement overlay
//!
//! Contains pathfinding logic specific to movement range calculation

use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};
use crate::{
    resources::{TileConfig, TileMap},
};

/// Find all tiles reachable within movement range using flood fill pathfinding
pub fn find_reachable_tiles(
    start_pos: IVec2,
    max_movement: i32,
    tile_config: &TileConfig,
    tile_map: &TileMap,
) -> Vec<IVec2> {
    let mut reachable = Vec::new();
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    
    // Start BFS from the unit's current position
    queue.push_back((start_pos, 0)); // (position, movement_cost)
    visited.insert(start_pos, 0);
    
    while let Some((current_pos, movement_cost)) = queue.pop_front() {
        // Check all 4 adjacent tiles (up, down, left, right)
        let directions = [
            IVec2::new(0, 1),   // up
            IVec2::new(0, -1),  // down
            IVec2::new(1, 0),   // right
            IVec2::new(-1, 0),  // left
        ];
        
        for direction in directions {
            let next_pos = current_pos + direction;
            let next_movement_cost = movement_cost + 1;
            
            // Skip if we've exceeded movement range
            if next_movement_cost > max_movement {
                continue;
            }
            
            // Skip if out of grid bounds
            if next_pos.x < 0 || next_pos.x >= tile_config.grid_size ||
               next_pos.y < 0 || next_pos.y >= tile_config.grid_size {
                continue;
            }
            
            // Skip if we've already visited this tile with equal or better cost
            if let Some(&existing_cost) = visited.get(&next_pos) {
                if existing_cost <= next_movement_cost {
                    continue;
                }
            }
            
            // Check what's at this tile using TileMap
            let has_enemy = tile_map.has_enemy(next_pos);
            let has_friendly = tile_map.has_unit(next_pos);
            
            // Skip if there's an enemy (impassable)
            if has_enemy {
                continue;
            }
            
            // Update visited with the new cost
            visited.insert(next_pos, next_movement_cost);
            
            // If it's an empty tile, it's a valid destination
            if !has_friendly {
                reachable.push(next_pos);
            }
            
            // Continue pathfinding from this position (friendly units are passable)
            queue.push_back((next_pos, next_movement_cost));
        }
    }
    
    reachable
}