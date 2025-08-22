// =============================================
// src/modes/battle/features/spawner_debug/systems_view.rs
// =============================================
use crate::domain::units::components::{TeamId, UnitGrid};
use crate::domain::units::events::UnitSpawnApplied;
use crate::infra::view_core::coords::GridGeometry;
use crate::infra::view_core::z_index::ZLayer;
use bevy::prelude::*;

/// 스폰된 유닛의 간단한 뷰(색 사각형)를 생성
#[derive(Component)]
pub struct UnitSprite;

pub fn spawn_unit_sprites_on_applied(
    mut commands: Commands,
    mut ev_in: EventReader<UnitSpawnApplied>,
    units: Query<(&UnitGrid, &TeamId)>,
    grid_geom: Res<GridGeometry>,
) {
    for ev in ev_in.read().copied() {
        let Ok((grid, team)) = units.get(ev.entity) else { continue; };

        let center = grid_geom.grid_center(grid.0);

        let color = match team {
            TeamId::Ally => Color::srgb(0.2, 0.6, 1.0),    // 파랑
            TeamId::Enemy => Color::srgb(1.0, 0.25, 0.25), // 빨강
        };
        let size = grid_geom.cell * 0.8; // 타일보다 살짝 작게

        // Bevy 0.14+ 호환: spawn tuple로 교체
        commands.spawn((
            Transform::from_xyz(center.x, center.y, ZLayer::Units.z()),
            Sprite {
                custom_size: Some(size),
                color,
                ..Default::default()
            },
            UnitSprite,
            Name::new(format!("unit_sprite@{},{}", grid.0.x, grid.0.y)),
        ));
        // (선택) 뷰-도메인 연결 컴포넌트를 넣고 싶으면 ev.entity를 저장하는 컴포넌트를 추가하세요.
    }
}
