//! AI movement system for enemy units

use bevy::prelude::*;
use crate::features::tiles::resources::TileMap;
use super::Enemy;

pub fn move_enemies_left(
    mut enemy_query: Query<(Entity, &mut Enemy), With<Enemy>>,
    mut tile_map: ResMut<TileMap>,
) {
    let mut enemies_to_move = Vec::new();
    
    for (enemy_entity, enemy) in enemy_query.iter() {
        let current_pos = enemy.tile_pos;
        enemies_to_move.push((enemy_entity, current_pos));
    }

    for (enemy_entity, current_pos) in enemies_to_move {
        let new_pos = IVec2::new(current_pos.x - 1, current_pos.y);
        
        if tile_map.is_valid_position(new_pos) && tile_map.is_empty(new_pos) {
            // Update TileMap
            tile_map.remove_entity(current_pos);
            tile_map.place_enemy(new_pos, enemy_entity);
            
            // Update Enemy component (Transform will be synced automatically)
            if let Ok((_, mut enemy)) = enemy_query.get_mut(enemy_entity) {
                enemy.tile_pos = new_pos;
            }
            
            info!("Enemy moved from {:?} to {:?}", current_pos, new_pos);
        }
    }
}