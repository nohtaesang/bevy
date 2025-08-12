//! Position synchronization for ally units

use bevy::prelude::*;
use crate::features::tiles::{tile_to_world_coords, TileConfig};
use super::Unit;

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