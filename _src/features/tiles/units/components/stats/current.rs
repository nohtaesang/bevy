// components/stats/current.rs
use bevy::prelude::*;

/// Current/runtime stats that change during gameplay (HP, AP, etc.)
#[derive(Component, Clone, Copy)]
pub struct CurrentStats {
    pub hp: i32,
    pub shield: i32,
    pub move_range: i32,
    pub actions_per_turn: i32,
}

impl CurrentStats {
    pub fn from_base(max_hp: i32, max_shield: i32, move_range: i32, actions_per_turn: i32) -> Self {
        Self {
            hp: max_hp,
            shield: max_shield,
            move_range,
            actions_per_turn,
        }
    }

    pub fn new(hp: i32, shield: i32, move_range: i32, actions_per_turn: i32) -> Self {
        Self {
            hp,
            shield,
            move_range,
            actions_per_turn,
        }
    }
}