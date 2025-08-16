// systems/recompute_effective.rs
use bevy::prelude::*;
use crate::features::units::components::stats::{BaseStats, AggregatedMods, EffectiveStats};

#[inline]
fn calc_i32(b: i32, add: i32, mul: f32, min_one: bool, min_zero: bool) -> i32 {
    let m = if mul <= 0.0 { 1.0 } else { mul };
    let mut v = (((b + add) as f32) * m).floor() as i32;
    if min_zero { v = v.max(0); }
    if min_one  { v = v.max(1); }
    v
}

#[inline]
fn calc_f32(b: f32, add: f32, mul: f32) -> f32 {
    let m = if mul <= 0.0 { 1.0 } else { mul };
    (b + add) * m
}

pub fn recompute_effective_stats(
    mut q: Query<(&BaseStats, &AggregatedMods, &mut EffectiveStats),
                 Or<(Changed<BaseStats>, Changed<AggregatedMods>)>>,
) {
    for (base, agg, mut eff) in &mut q {
        eff.damage           = calc_i32(base.damage,      agg.stats.damage_add,     agg.stats.damage_mul,     false, true);
        eff.move_range       = calc_i32(base.move_range,  agg.stats.move_add,       agg.stats.move_mul,       false, true);
        eff.actions_per_turn = calc_i32(base.actions_per_turn, agg.stats.actions_add, agg.stats.actions_mul,  true,  true);

        // ✅ 사거리: 각각 계산 후 일관성 보정
        let minr = calc_i32(base.min_range, agg.stats.min_range_add, agg.stats.min_range_mul, false, true);
        let maxr = calc_i32(base.max_range, agg.stats.max_range_add, agg.stats.max_range_mul, false, true);
        eff.min_range = minr.min(maxr);   // min <= max 보장
        eff.max_range = maxr.max(minr);

        eff.max_hp      = calc_i32(base.max_hp,      agg.stats.max_hp_add,     agg.stats.max_hp_mul,     true,  true);
        eff.max_shield  = calc_i32(base.max_shield,  agg.stats.max_shield_add, agg.stats.max_shield_mul, false, true);

        let cc = calc_f32(base.crit_chance,     agg.stats.crit_chance_add, agg.stats.crit_chance_mul);
        eff.crit_chance     = cc.clamp(0.0, 1.0);

        let cm = calc_f32(base.crit_multiplier, agg.stats.crit_mult_add,   agg.stats.crit_mult_mul);
        eff.crit_multiplier = cm.max(1.0);
    }
}
