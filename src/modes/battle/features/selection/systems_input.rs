// =============================================
// src/modes/battle/features/selection/systems_input.rs
// =============================================
use crate::domain::units::components::{Unit, UnitGrid};
use crate::modes::battle::features::overlays::hover_tile::resources::HoverTile;
use crate::modes::battle::features::selection::resources::Selected;
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;

/// 좌클릭으로 현재 호버 타일을 선택.
/// 같은 칸에 유닛이 있으면 unit에도 설정, 없으면 unit=None.
pub fn select_on_left_click(
    buttons: Res<ButtonInput<MouseButton>>,
    hover: Res<HoverTile>,
    mut selected: ResMut<Selected>,
    q_units: Query<(Entity, &UnitGrid), With<Unit>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    // 타일 선택 (맵 밖이면 해제)
    selected.tile = hover.grid;

    // 유닛 선택
    selected.unit = hover.grid.and_then(|gp| {
        q_units
            .iter()
            .find_map(|(e, ug)| if ug.0 == gp { Some(e) } else { None })
    });
}
