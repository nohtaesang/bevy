//! Shared types and utilities for units

use bevy::prelude::*;

/// Attack direction types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttackDirection {
    /// Can attack in 4 cardinal directions (up, down, left, right)
    Cardinal,
    /// Can attack in 8 directions (cardinal + diagonals)
    EightWay,
}

/// Attack type that determines line of sight requirements
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttackType {
    /// Direct fire - cannot shoot over units (requires clear line of sight)
    Direct,
    /// Indirect fire - can shoot over units (no line of sight required)
    Indirect,
}

/// Attack range with minimum and maximum distance
#[derive(Debug, Clone, Copy)]
pub struct AttackRange {
    pub min: i32,
    pub max: i32,
}

impl AttackRange {
    pub fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }
    
    /// Check if a distance is within attack range
    pub fn is_in_range(&self, distance: i32) -> bool {
        distance >= self.min && distance <= self.max
    }
}