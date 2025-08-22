// src/infra/view_core/coords/plugin.rs
//! 좌표계 플러그인

use super::systems::sync_grid_geometry;
use super::types::GridGeometry;
use crate::app::state::ModeState;
use bevy::prelude::*;

/// 그리드↔월드 좌표 변환 시스템 플러그인
pub struct CoordsPlugin;

impl Plugin for CoordsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridGeometry>().add_systems(
            PostUpdate,
            sync_grid_geometry
                .run_if(in_state(ModeState::Battle))
                .in_set(CoordsSet::Sync),
        );
    }
}

/// 좌표계 시스템 실행 순서 제어용 SystemSet
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CoordsSet {
    /// GridGeometry 동기화
    Sync,
}
