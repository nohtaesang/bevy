//! Selected overlay system

use bevy::prelude::*;
use crate::{
    features::tiles::{tile_to_world_coords, SelectionCtx, TileConfig},
};
use super::SelectedOverlay;

pub fn selected_overlay_system(
    selection_ctx: Res<SelectionCtx>,
    tile_config: Res<TileConfig>,
    mut overlay_query: Query<(&mut Transform, &mut Visibility), With<SelectedOverlay>>,
) {
    if let Ok((mut transform, mut visibility)) = overlay_query.single_mut() {
        if let Some(tile_pos) = selection_ctx.tile {
            let world_pos = tile_to_world_coords(tile_pos.x, tile_pos.y, &tile_config);
            
            transform.translation.x = world_pos.x;
            transform.translation.y = world_pos.y;
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}