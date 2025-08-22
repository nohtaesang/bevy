// =============================================
// src/modes/battle/features/spawner_debug/systems_input.rs
// =============================================
use crate::domain::units::components::TeamId;
use crate::domain::units::events::UnitSpawnRequested;
use crate::modes::battle::features::overlays::hover_tile::resources::HoverTile;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

/// 키 입력(U/I) + Hover 타일 → 스폰 요청 이벤트 발행
pub fn emit_spawn_requests_from_input(
    keys: Res<ButtonInput<KeyCode>>,
    hover: Res<HoverTile>,
    mut ev_out: EventWriter<UnitSpawnRequested>,
) {
    let Some(at) = hover.grid else { return; };

    if keys.just_pressed(KeyCode::KeyU) {
        ev_out.write(UnitSpawnRequested {
            team: TeamId::Ally,
            at,
        });
    }
    if keys.just_pressed(KeyCode::KeyI) {
        ev_out.write(UnitSpawnRequested {
            team: TeamId::Enemy,
            at,
        });
    }
}
