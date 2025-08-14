use bevy::prelude::*;
use crate::features::tiles::{
    core::{TileConfig, events::Team},
    units::{
        bundles::UnitBundle,
        components::{
            stats::BaseStats,
            attack_profile::{AttackProfile, AttackDirection, AttackType},
        },
    },
};

pub fn make_basic_ally_bundle(pos: IVec2, team: Team, cfg: &TileConfig) -> UnitBundle {
    let base = BaseStats::new(3, 3, 1, 1, 3, 100, 0, 0.10, 1.5);
    let attack = AttackProfile::new(AttackDirection::Cardinal, AttackType::Direct);
    UnitBundle::new_on_grid(pos, team, base, attack, cfg)
}
