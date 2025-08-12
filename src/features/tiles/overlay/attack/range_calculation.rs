//! Attack range calculation for attack overlay
//!
//! Contains logic for calculating which tiles can be attacked

use bevy::prelude::*;
use std::collections::HashSet;
use crate::{
    features::units::{AttackDirection, AttackType, AttackRange},
    resources::{TileConfig, TileMap},
};

/// Find all tiles that can be attacked from the given position
pub fn find_attackable_tiles(
    attacker_pos: IVec2,
    attack_direction: AttackDirection,
    attack_type: AttackType,
    attack_range: &AttackRange,
    tile_config: &TileConfig,
    tile_map: &TileMap,
) -> Vec<IVec2> {
    let mut attackable = Vec::new();
    let mut checked = HashSet::new();
    
    // Get direction vectors based on attack direction type
    let directions = match attack_direction {
        AttackDirection::Cardinal => vec![
            IVec2::new(0, 1),   // up
            IVec2::new(0, -1),  // down
            IVec2::new(1, 0),   // right
            IVec2::new(-1, 0),  // left
        ],
        AttackDirection::EightWay => vec![
            IVec2::new(0, 1),   // up
            IVec2::new(0, -1),  // down
            IVec2::new(1, 0),   // right
            IVec2::new(-1, 0),  // left
            IVec2::new(1, 1),   // up-right
            IVec2::new(-1, 1),  // up-left
            IVec2::new(1, -1),  // down-right
            IVec2::new(-1, -1), // down-left
        ],
    };
    
    // Check each direction
    for direction in directions {
        // Check each distance in the range
        for distance in attack_range.min..=attack_range.max {
            let target_pos = attacker_pos + direction * distance;
            
            // Skip if out of bounds
            if target_pos.x < 0 || target_pos.x >= tile_config.grid_size ||
               target_pos.y < 0 || target_pos.y >= tile_config.grid_size {
                continue;
            }
            
            // Skip if already checked
            if checked.contains(&target_pos) {
                continue;
            }
            checked.insert(target_pos);
            
            // For direct fire, check if line of sight is blocked
            if attack_type == AttackType::Direct {
                if !has_line_of_sight(attacker_pos, target_pos, tile_map) {
                    // If blocked, can't attack this tile or any further tiles in this direction
                    break;
                }
            }
            
            // This tile can be attacked
            attackable.push(target_pos);
        }
    }
    
    attackable
}

/// Check if there's a clear line of sight between two positions
fn has_line_of_sight(
    from: IVec2,
    to: IVec2,
    tile_map: &TileMap,
) -> bool {
    let diff = to - from;
    let distance = diff.x.abs().max(diff.y.abs());
    
    if distance <= 1 {
        return true; // Adjacent tiles always have line of sight
    }
    
    // Check each tile along the path (excluding start and end)
    for i in 1..distance {
        let t = i as f32 / distance as f32;
        let check_pos = IVec2::new(
            (from.x as f32 + diff.x as f32 * t).round() as i32,
            (from.y as f32 + diff.y as f32 * t).round() as i32,
        );
        
        // Check if there's a unit or enemy blocking the path using TileMap
        if !tile_map.is_empty(check_pos) {
            return false; // Path is blocked
        }
    }
    
    true
}