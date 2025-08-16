//! Attack range calculation for ally units
//!
//! Calculates valid attack positions when an ally unit is selected

use bevy::prelude::*;
use std::collections::HashSet;
use crate::features::tiles::{
    core::{Team, components::TileCoords, TileConfig, TileMap},
    selection::SelectionCtx,
    units::{
        bundles::UnitMarker,
        components::{
            stats::CurrentStats,
            attack_profile::{AttackProfile, AttackDirection, AttackType},
        },
    },
    interaction::AttackValidation,
};

/// System that updates attack validation when an ally is selected
pub fn update_ally_attack_range(
    selection_ctx: Res<SelectionCtx>,
    tile_config: Res<TileConfig>,
    tile_map: Res<TileMap>,
    unit_query: Query<(&Team, &TileCoords, &CurrentStats, &AttackProfile), With<UnitMarker>>,
    mut attack_validation: ResMut<AttackValidation>,
) {
    // Clear existing validation
    attack_validation.clear();
    
    // Check if an ally unit is selected
    let Some(selected_entity) = selection_ctx.selected_unit else {
        return;
    };
    
    // Get unit information
    let Ok((team, tile_coords, current_stats, attack_profile)) = unit_query.get(selected_entity) else {
        return;
    };
    
    // Only calculate for player units
    if *team != Team::Player {
        return;
    }
    
    // Check if unit can attack
    if current_stats.actions_per_turn <= 0 {
        return;
    }
    
    // Calculate attack range
    let unit_pos = IVec2::new(tile_coords.x, tile_coords.y);
    let valid_attacks = calculate_attack_range(
        unit_pos,
        attack_profile,
        2, // min_range - hardcoded for now
        3, // max_range - hardcoded for now
        &tile_config,
        &tile_map,
    );
    
    // Update the validation resource
    attack_validation.set_valid_attacks(valid_attacks);
}

/// Calculate valid attack positions based on unit's attack profile
fn calculate_attack_range(
    unit_pos: IVec2,
    attack_profile: &AttackProfile,
    min_range: i32,
    max_range: i32,
    tile_config: &TileConfig,
    tile_map: &TileMap,
) -> HashSet<IVec2> {
    let mut valid_attacks = HashSet::new();
    
    // Generate potential attack positions based on attack direction
    let potential_positions = match attack_profile.direction {
        AttackDirection::Cardinal => generate_four_way_positions(unit_pos, min_range, max_range),
        AttackDirection::EightWay => generate_eight_way_positions(unit_pos, min_range, max_range),
    };
    
    // Filter positions based on attack type and tile validity
    for pos in potential_positions {
        // Check if position is within map bounds
        if pos.x < 0 || pos.x >= tile_config.grid_size || 
           pos.y < 0 || pos.y >= tile_config.grid_size {
            continue;
        }
        
        // Check attack type requirements
        let is_valid = match attack_profile.kind {
            AttackType::Direct => {
                // Direct attacks need line of sight (simplified: no obstacles between)
                has_line_of_sight(unit_pos, pos, tile_map)
            },
            AttackType::Indirect => {
                // Indirect attacks can go over obstacles
                true
            },
        };
        
        if is_valid {
            // Check if there's an enemy at this position (optional - for highlighting)
            // For now, we'll add all valid attack positions
            valid_attacks.insert(pos);
        }
    }
    
    valid_attacks
}

/// Generate positions for 4-way attacks (cardinal directions)
fn generate_four_way_positions(center: IVec2, min_range: i32, max_range: i32) -> Vec<IVec2> {
    let mut positions = Vec::new();
    
    // North, South, East, West
    for distance in min_range..=max_range {
        positions.push(IVec2::new(center.x + distance, center.y));  // East
        positions.push(IVec2::new(center.x - distance, center.y));  // West
        positions.push(IVec2::new(center.x, center.y + distance));  // North
        positions.push(IVec2::new(center.x, center.y - distance));  // South
    }
    
    positions
}

/// Generate positions for 8-way attacks (including diagonals)
fn generate_eight_way_positions(center: IVec2, min_range: i32, max_range: i32) -> Vec<IVec2> {
    let mut positions = Vec::new();
    
    // Generate all positions within Manhattan distance
    for dx in -max_range..=max_range {
        for dy in -max_range..=max_range {
            let distance = dx.abs().max(dy.abs()); // Chebyshev distance for 8-way
            
            if distance >= min_range && distance <= max_range {
                positions.push(IVec2::new(center.x + dx, center.y + dy));
            }
        }
    }
    
    positions
}

/// Check if there's a clear line of sight between two positions
fn has_line_of_sight(from: IVec2, to: IVec2, _tile_map: &TileMap) -> bool {
    // Simplified line of sight check
    // TODO: Implement proper Bresenham's line algorithm to check for obstacles
    // For now, always return true for direct attacks
    
    // Basic check: ensure we're not trying to attack through the diagonal of a blocked tile
    let dx = (to.x - from.x).signum();
    let dy = (to.y - from.y).signum();
    
    // If moving diagonally, check that we're not cutting through corners
    if dx != 0 && dy != 0 {
        // For diagonal movement, at least one of the adjacent tiles should be free
        // This is a simplified check - a full implementation would trace the entire path
        return true;
    }
    
    true
}