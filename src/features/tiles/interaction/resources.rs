//! Interaction resources
//!
//! Resources for tracking user interaction state

use bevy::prelude::*;

/// Resource tracking which tile the cursor is hovering over
/// Only stores tile coordinates - world position is calculated when needed
#[derive(Resource, Default, Debug, Clone, PartialEq)]
pub struct HoverTile {
    /// Current tile position under cursor (None if outside grid)
    pub tile_pos: Option<IVec2>,
}

impl HoverTile {
    /// Clear the hover state
    pub fn clear(&mut self) {
        self.tile_pos = None;
    }
    
    /// Update hover position
    pub fn set(&mut self, tile_pos: IVec2) {
        self.tile_pos = Some(tile_pos);
    }
    
    /// Check if hovering over a tile
    pub fn is_hovering(&self) -> bool {
        self.tile_pos.is_some()
    }
}