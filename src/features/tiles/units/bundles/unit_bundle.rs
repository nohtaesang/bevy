use bevy::prelude::*;
use crate::features::tiles::{
    core::{
        components::TileCoords,
        resources::{TileConfig},
        systems::tile_to_world_coords,
        events::Team,
    },
    units::components::{
        stats::{BaseStats, AggregatedMods, EffectiveStats, CurrentStats},
        attack_profile::AttackProfile,
    },
};

#[derive(Component)]
pub struct UnitMarker;

#[derive(Bundle)]
pub struct UnitBundle {
    pub name: Name,
    pub team: Team,
    pub marker: UnitMarker,

    pub tile_pos: TileCoords,

    pub base: BaseStats,
    pub mods: AggregatedMods,
    pub eff: EffectiveStats,
    pub cur: CurrentStats,

    pub attack: AttackProfile,

    pub transform: Transform,      // 부모(로직) Transform
    pub global: GlobalTransform,
}

impl UnitBundle {
    pub fn new_on_grid(
        pos: IVec2,
        team: Team,
        base: BaseStats,
        attack: AttackProfile,
        cfg: &TileConfig,
    ) -> Self {
        let wp = tile_to_world_coords(pos.x, pos.y, cfg);
        Self {
            name: Name::new("Unit"),
            team,
            marker: UnitMarker,
            tile_pos: TileCoords { x: pos.x, y: pos.y },

            base,
            mods: AggregatedMods::default(),
            eff: EffectiveStats::default(),
            cur: CurrentStats::from_base(
                base.max_hp, base.max_shield, base.move_range, base.actions_per_turn
            ),

            attack,
            transform: Transform::from_xyz(wp.x, wp.y, 0.0),
            global: GlobalTransform::default(),
        }
    }
}
