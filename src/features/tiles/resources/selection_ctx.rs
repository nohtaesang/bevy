//! Selection context resource for tile-based gameplay
//!
//! Contains the current selection state and coordinates

use bevy::prelude::*;

/// Selection context resource to hold coordinates and entity references
#[derive(Resource, Default, Debug)]
pub struct SelectionCtx {
    pub tile: Option<IVec2>,
    pub selected_unit: Option<Entity>,
    pub selected_enemy: Option<Entity>,
}