//! Ally (player-controlled) unit components

use bevy::prelude::*;
use crate::features::units::shared::{AttackDirection, AttackType, AttackRange};

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