// components/stats/effective.rs
use bevy::prelude::*;

#[derive(Component, Clone, Copy, Default)]
pub struct EffectiveStats {
    pub damage: i32,
    pub move_range: i32,
    pub actions_per_turn: i32,

    pub min_range: i32,            // ✅
    pub max_range: i32,            // ✅

    pub max_hp: i32,
    pub max_shield: i32,
    pub crit_chance: f32,
    pub crit_multiplier: f32,
}
