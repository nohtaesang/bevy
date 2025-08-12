//! Movement overlay state resource
//!
//! Shared state for movement validation

use bevy::prelude::*;
use std::collections::HashSet;

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
}