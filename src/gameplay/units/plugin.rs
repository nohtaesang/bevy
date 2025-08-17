// src/gameplay/units/plugin.rs
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use crate::app::state::AppState;
use crate::gameplay::tiles::plugin::TilesSet;
use crate::gameplay::units::systems::spawn_from_assets::has_pending; // TilesSet가 pub이어야 함

use super::spec::UnitSpec;
use super::assets::PendingUnitLoads;
use super::systems::spawn_from_assets::{ process_loaded_units};

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app
            // RON 스펙 로더 등록
            .add_plugins(RonAssetPlugin::<UnitSpec>::new(&["ron"]))
            // 리소스
            .init_resource::<PendingUnitLoads>()
            // 로드 처리: 타일 적용과 같은 프레임에 일어나게 PreUpdate에서,
            // TilesSet::ApplyCommands 전에 실행
            .add_systems(
                PreUpdate,
                process_loaded_units
                    .run_if(in_state(AppState::Battle))
                    .run_if(has_pending)
                    .before(TilesSet::ApplyCommands),
            );
    }
}
