// ===============================
// src/modes/battle/features/map_view/plugin.rs
// ===============================
use bevy::prelude::*;

use super::view::{spawn_tiles_on_map_initialized, TileViewConfig};
use crate::app::state::ModeState;

/// Renders tiles when the domain map finishes initializing.
pub struct MapViewPlugin;

impl Plugin for MapViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileViewConfig>()
            .add_systems(
                Update,
                spawn_tiles_on_map_initialized.run_if(in_state(ModeState::Battle)),
            );
    }
}
