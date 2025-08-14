//! Unit components module
//!
//! This module contains Ally, Enemy, Health, AP and related unit components

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

/// Ally (player-controlled) unit marker component
#[derive(Component, Debug)]
pub struct Ally;

/// Enemy (AI-controlled) unit marker component  
#[derive(Component, Debug)]
pub struct EnemyMarker;

/// Health component for units
#[derive(Component, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }
    
    pub fn ratio(&self) -> f32 {
        self.current as f32 / self.max as f32
    }
}

/// Action Points component for units
#[derive(Component, Debug)]
pub struct AP {
    pub movement_range: i32,
    pub max_movement_range: i32,
    pub attack_count: i32,
    pub max_attack_count: i32,
}

impl AP {
    pub fn new(movement_range: i32, attack_count: i32) -> Self {
        Self {
            movement_range,
            max_movement_range: movement_range,
            attack_count,
            max_attack_count: attack_count,
        }
    }
    
    /// Reset AP for a new turn
    pub fn reset_turn(&mut self) {
        self.movement_range = self.max_movement_range;
        self.attack_count = self.max_attack_count;
    }
    
    /// Check if unit can move
    pub fn can_move(&self) -> bool {
        self.movement_range > 0
    }
    
    /// Check if unit can attack
    pub fn can_attack(&self) -> bool {
        self.attack_count > 0
    }
    
    /// Use movement
    pub fn use_movement(&mut self, amount: i32) {
        self.movement_range = (self.movement_range - amount).max(0);
    }
    
    /// Use attack
    pub fn use_attack(&mut self) {
        self.attack_count = (self.attack_count - 1).max(0);
    }
}

/// Legacy Unit component for backward compatibility
#[derive(Component, Debug)]
pub struct Unit {
    pub tile_pos: IVec2,
    pub health: i32,
    pub max_health: i32,
    pub attack_power: i32,
    pub movement_range: i32,
    pub max_movement_range: i32,
    
    // Attack capabilities
    pub attack_count: i32,
    pub max_attack_count: i32,
    pub attack_direction: AttackDirection,
    pub attack_type: AttackType,
    pub attack_range: AttackRange,
}

impl Unit {
    /// Create a new unit with specified attack capabilities
    pub fn new(
        tile_pos: IVec2, 
        attack_direction: AttackDirection,
        attack_type: AttackType,
        attack_range: AttackRange,
    ) -> Self {
        Self {
            tile_pos,
            health: 100,
            max_health: 100,
            attack_power: 10,
            movement_range: 3,
            max_movement_range: 3,
            attack_count: 1,
            max_attack_count: 1,
            attack_direction,
            attack_type,
            attack_range,
        }
    }
    
    /// Reset movement and attack counts for a new turn
    pub fn reset_turn_actions(&mut self) {
        self.movement_range = self.max_movement_range;
        self.attack_count = self.max_attack_count;
    }
    
    /// Check if unit can move (has movement range left)
    pub fn can_move(&self) -> bool {
        self.movement_range > 0
    }
    
    /// Check if unit can attack (has attack count left)
    pub fn can_attack(&self) -> bool {
        self.attack_count > 0
    }
    
    /// Use movement (reduces movement_range by amount)
    pub fn use_movement(&mut self, amount: i32) {
        self.movement_range = (self.movement_range - amount).max(0);
    }
    
    /// Use attack (reduces attack_count by 1)
    pub fn use_attack(&mut self) {
        self.attack_count = (self.attack_count - 1).max(0);
    }
}

/// Legacy Enemy component for backward compatibility
#[derive(Component, Debug)]
pub struct LegacyEnemy {
    pub tile_pos: IVec2,
    pub health: i32,
    pub max_health: i32,
    pub attack_power: i32,
}

impl LegacyEnemy {
    pub fn new(tile_pos: IVec2) -> Self {
        Self {
            tile_pos,
            health: 50,
            max_health: 50,
            attack_power: 5,
        }
    }
}

// Type alias for backward compatibility
pub type Enemy = LegacyEnemy;