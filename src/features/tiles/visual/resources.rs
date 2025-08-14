//! Visual overlay resources and state
//!
//! This module contains OverlayState and validation resources

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

/// Local state for movement overlay system
#[derive(Default)]
pub struct MovementOverlayState {
    pub current_overlays: Vec<Entity>,
    pub valid_moves: HashSet<IVec2>,  // Cache valid move positions
    pub last_unit_pos: Option<IVec2>,
    pub last_movement_range: Option<i32>,
}

/// Local state for attack overlay system
#[derive(Default)]
pub struct AttackOverlayState {
    pub current_overlays: Vec<Entity>,
    pub valid_attacks: HashSet<IVec2>,  // Cache valid attack positions
    pub last_unit_pos: Option<IVec2>,
    pub last_attack_count: Option<i32>,
}