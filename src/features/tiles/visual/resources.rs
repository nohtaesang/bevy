//! Visual overlay resources and state
//!
//! This module contains OverlayState resources

use bevy::prelude::*;
use std::collections::HashSet;

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