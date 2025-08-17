use bevy::prelude::*;

use super::camera::CameraPlugin;
use super::tiles::TilesViewPlugin;

/// 뷰(렌더/UI/이펙트 등)를 묶는 상위 플러그인.
/// - 카메라 전용 플러그인 등록
/// - Battle 진입 시 2D 카메라가 없으면 자동 생성
pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app
            // 카메라 전용 플러그인(내부에서 Battle 상태 run_if 사용 권장)
            .add_plugins(CameraPlugin)
            .add_plugins(TilesViewPlugin);
    }
}

