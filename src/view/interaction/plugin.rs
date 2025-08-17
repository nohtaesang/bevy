use bevy::prelude::*;

use crate::app::state::AppState;

use super::{
    resources::TileOverlayConfig,
    systems::{spawn_tile_overlays_once, sync_hover_overlay, sync_selected_overlay},
};

/// 타일 Hover/Selection 테두리 오버레이 플러그인
pub struct TileOverlayViewPlugin;

impl Plugin for TileOverlayViewPlugin {
    fn build(&self, app: &mut App) {
        app
            // 설정 리소스
            .init_resource::<TileOverlayConfig>()
            // Battle에 들어올 때 한 번만 생성
            .add_systems(OnEnter(AppState::Battle), spawn_tile_overlays_once)
            // Battle 상태일 때만 매 프레임 동기화
            .add_systems(
                Update,
                (sync_hover_overlay, sync_selected_overlay)
                    .run_if(in_state(AppState::Battle)),
            );

    }
}
