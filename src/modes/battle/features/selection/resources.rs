// =============================================
// src/modes/battle/features/selection/resources.rs
// =============================================
use crate::domain::map::grid_index::GridPos;
use bevy::prelude::*;

/// 현재 선택 상태 (타일/유닛). UI 상태라 SSOT가 아님.
#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct Selected {
    pub tile: Option<GridPos>,
    pub unit: Option<Entity>,
}

/// 선택 하이라이트 스타일
#[derive(Resource)]
pub struct SelectionHighlightConfig {
    pub color: Color,
    pub size_scale: f32,
}
impl Default for SelectionHighlightConfig {
    fn default() -> Self {
        Self {
            color: Color::srgb(1.0, 0.9, 0.2).with_alpha(0.35),
            size_scale: 1.0,
        }
    }
}
