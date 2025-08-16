//! Interaction resources
//!
//! Resources for tracking user interaction state and range validation

use bevy::prelude::*;
use std::collections::HashSet;

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

/// Resource that stores current valid movement positions
#[derive(Resource, Default)]
pub struct MovementValidation {
    pub valid_moves: HashSet<IVec2>,
}

impl MovementValidation {
    /// Check if a position is a valid move target
    pub fn is_valid_move(&self, pos: IVec2) -> bool {
        self.valid_moves.contains(&pos)
    }
    
    /// Update valid moves
    pub fn set_valid_moves(&mut self, moves: HashSet<IVec2>) {
        self.valid_moves = moves;
    }
    
    /// Clear all valid moves
    pub fn clear(&mut self) {
        self.valid_moves.clear();
    }
    
    /// Get valid moves for debugging
    pub fn get_valid_moves(&self) -> &HashSet<IVec2> {
        &self.valid_moves
    }
}

/// Resource that stores current valid attack positions
#[derive(Resource, Default)]
pub struct AttackValidation {
    pub valid_attacks: HashSet<IVec2>,
}

impl AttackValidation {
    /// Check if a position is a valid attack target
    pub fn is_valid_attack(&self, pos: IVec2) -> bool {
        self.valid_attacks.contains(&pos)
    }
    
    /// Update valid attacks
    pub fn set_valid_attacks(&mut self, attacks: HashSet<IVec2>) {
        self.valid_attacks = attacks;
    }
    
    /// Clear all valid attacks
    pub fn clear(&mut self) {
        self.valid_attacks.clear();
    }
    
    /// Get valid attacks for debugging
    pub fn get_valid_attacks(&self) -> &HashSet<IVec2> {
        &self.valid_attacks
    }
}