//! AI systems for enemy units
//!
//! This module handles AI movement and behavior for enemy units

use bevy::prelude::*;
use crate::features::tiles::{
    core::{TileMap, TileMoved, Team, MoveOutcome},
    units::components::Enemy,
};

/// AI movement system for enemy units
pub fn move_enemies_left(
    mut enemy_query: Query<(Entity, &mut Enemy, &Team), With<Enemy>>,
    mut tile_map: ResMut<TileMap>,
    mut tile_moved: EventWriter<TileMoved>,
) {
    let mut enemies_to_move = Vec::new();
    
    for (enemy_entity, enemy, team) in enemy_query.iter() {
        let current_pos = enemy.tile_pos;
        enemies_to_move.push((enemy_entity, current_pos, *team));
    }

    for (enemy_entity, current_pos, team) in enemies_to_move {
        let new_pos = IVec2::new(current_pos.x - 1, current_pos.y);
        
        // Try to move using TileMap first (SSOT for tile occupancy)
        match tile_map.move_unit(current_pos, new_pos) {
            MoveOutcome::Moved { entity: moved_entity } => {
                // TileMap update succeeded - now update other systems
                
                // Update Enemy component (Transform will be synced automatically)
                if let Ok((_, mut enemy, _)) = enemy_query.get_mut(enemy_entity) {
                    enemy.tile_pos = new_pos;
                }
                
                // Emit event only after TileMap update succeeded (use actual team from component)
                tile_moved.send(TileMoved {
                    entity: moved_entity,
                    from: current_pos,
                    to: new_pos,
                    team,
                });
                
                info!("Enemy moved from {:?} to {:?}", current_pos, new_pos);
            },
            MoveOutcome::Blocked => {
                // Enemy movement blocked, could implement different AI behavior here
                info!("Enemy at {:?} blocked by obstacle/unit at {:?}", current_pos, new_pos);
            },
            MoveOutcome::OutOfBounds => {
                // Enemy tried to move out of bounds
                info!("Enemy at {:?} tried to move out of bounds to {:?}", current_pos, new_pos);
            },
            MoveOutcome::EmptyFrom => {
                // This shouldn't happen, but log it
                warn!("Enemy entity {:?} not found at expected position {:?}", enemy_entity, current_pos);
            },
        }
    }
}