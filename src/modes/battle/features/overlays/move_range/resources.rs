// =============================================
// src/modes/battle/features/overlays/move_range/resources.rs
// =============================================
use crate::domain::map::grid_index::GridPos;
use bevy::prelude::*;

/// 선택된 유닛 기준으로 계산된 이동 가능 타일 집합
#[derive(Resource, Default, Debug, Clone)]
pub struct MoveRangeOverlay {
    pub of_unit: Option<Entity>,
    pub tiles: Vec<GridPos>,
}

/// 색/스타일 설정
#[derive(Resource)]
pub struct MoveRangeConfig {
    pub ally_color: Color,
    pub enemy_color: Color,
    pub size_scale: f32,
}
impl Default for MoveRangeConfig {
    fn default() -> Self {
        Self {
            ally_color: Color::srgb(0.2, 0.8, 1.0).with_alpha(0.25), // 시안 계열
            enemy_color: Color::srgb(1.0, 0.4, 0.3).with_alpha(0.25), // 주황/빨강 계열
            size_scale: 0.95,
        }
    }
}
