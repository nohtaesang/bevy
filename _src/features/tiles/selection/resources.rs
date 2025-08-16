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
    pub target_tile: Option<IVec2>,
}

impl SelectionCtx {
    /// Clear unit selection
    pub fn clear_unit_selection(&mut self) {
        self.selected_unit = None;
    }
    
    /// Clear enemy selection
    pub fn clear_enemy_selection(&mut self) {
        self.selected_enemy = None;
    }
    
    /// Clear all selections
    pub fn clear_all(&mut self) {
        *self = Self::default();
    }
}