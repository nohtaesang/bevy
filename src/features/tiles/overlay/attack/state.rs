//! Attack overlay state resource
//!
//! Shared state for attack validation

use bevy::prelude::*;
use std::collections::HashSet;

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