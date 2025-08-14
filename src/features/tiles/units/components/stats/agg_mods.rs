// components/stats/agg_mods.rs
use bevy::prelude::*;

/// (Base + add) * mul
#[derive(Clone, Copy, Debug)]
pub struct StatAddMul {
    pub damage_add: i32,      pub damage_mul: f32,
    pub move_add: i32,        pub move_mul: f32,
    pub actions_add: i32,     pub actions_mul: f32,

    pub min_range_add: i32,   pub min_range_mul: f32,   // ✅
    pub max_range_add: i32,   pub max_range_mul: f32,   // ✅

    pub max_hp_add: i32,      pub max_hp_mul: f32,
    pub max_shield_add: i32,  pub max_shield_mul: f32,
    pub crit_chance_add: f32, pub crit_chance_mul: f32,
    pub crit_mult_add: f32,   pub crit_mult_mul: f32,
}

impl Default for StatAddMul {
    fn default() -> Self {
        Self {
            damage_add: 0,      damage_mul: 1.0,
            move_add: 0,        move_mul: 1.0,
            actions_add: 0,     actions_mul: 1.0,

            min_range_add: 0,   min_range_mul: 1.0,   // ✅
            max_range_add: 0,   max_range_mul: 1.0,   // ✅

            max_hp_add: 0,      max_hp_mul: 1.0,
            max_shield_add: 0,  max_shield_mul: 1.0,
            crit_chance_add: 0.0, crit_chance_mul: 1.0,
            crit_mult_add: 0.0,   crit_mult_mul: 1.0,
        }
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct AggregatedMods {
    pub stats: StatAddMul,
}
