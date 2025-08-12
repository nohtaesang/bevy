//! Position synchronization for enemy units

use bevy::prelude::*;
use crate::features::tiles::{tile_to_world_coords, TileConfig};
use super::Enemy;

/// System that syncs Transform positions based on Enemy tile_pos
pub fn sync_enemy_positions(
    mut enemy_query: Query<(&Enemy, &mut Transform), (With<Enemy>, Changed<Enemy>)>,
    tile_config: Res<TileConfig>,
) {
    for (enemy, mut transform) in enemy_query.iter_mut() {
        let world_pos = tile_to_world_coords(enemy.tile_pos.x, enemy.tile_pos.y, &tile_config);
        transform.translation.x = world_pos.x;
        transform.translation.y = world_pos.y;
    }
}