//! Unit movement systems
//!
//! This module handles unit movement and position synchronization

use bevy::prelude::*;
use crate::features::tiles::{
    core::{TileConfig, tile_to_world_coords},
    units::components::{Unit, Enemy},
};

/// System that syncs Transform positions based on Unit tile_pos
pub fn sync_unit_positions(
    mut unit_query: Query<(&Unit, &mut Transform), (With<Unit>, Changed<Unit>)>,
    tile_config: Res<TileConfig>,
) {
    for (unit, mut transform) in unit_query.iter_mut() {
        let world_pos = tile_to_world_coords(unit.tile_pos.x, unit.tile_pos.y, &tile_config);
        transform.translation.x = world_pos.x;
        transform.translation.y = world_pos.y;
    }
}

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