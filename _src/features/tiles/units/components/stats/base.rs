// components/stats/base.rs
use bevy::prelude::*;

/// 변화가 드문 "설계상 기본 능력치"
#[derive(Component, Clone, Copy)]
pub struct BaseStats {
    pub damage: i32,
    pub move_range: i32,
    pub actions_per_turn: i32,
    pub min_range: i32,          
    pub max_range: i32,          
    pub max_hp: i32,
    pub max_shield: i32,
    pub crit_chance: f32,         // 0.0..=1.0
    pub crit_multiplier: f32,     // >= 1.0
}

impl BaseStats {
    pub fn new(
        damage: i32,
        move_range: i32,
        actions_per_turn: i32,
        min_range: i32,
        max_range: i32,
        max_hp: i32,
        max_shield: i32,
        crit_chance: f32,
        crit_multiplier: f32,
    ) -> Self {
        Self {
            damage,
            move_range,
            actions_per_turn,
            min_range,
            max_range,
            max_hp,
            max_shield,
            crit_chance,
            crit_multiplier,
        }
    }
}
