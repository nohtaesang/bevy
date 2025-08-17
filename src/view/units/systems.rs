use bevy::prelude::*;

use crate::gameplay::{
    tiles::{
        components::GridPos,
        resources::{TileConfig},
    },
    units::{
        components::{Unit, TeamId},
        resources::{Teams, TeamColors},
    },
};
use crate::view::tiles::resources::TileViewConfig;

use super::{
    components::{UnitSprite, UnitSpriteBundle, UnitSpriteLink, UnitVisual},
    resources::UnitViewConfig,
};

/// 팀 컬러 선택(Teams 매핑 기준)
fn team_color(team: TeamId, teams: &Teams, colors: &TeamColors) -> Color {
    if team == teams.ally { colors.ally }
    else if team == teams.enemy { colors.enemy }
    else { Color::srgb(0.9, 0.9, 0.9) } // fallback
}

/// 새로 생긴 Unit(혹은 아직 스프라이트가 없는 Unit)에 자식 스프라이트 생성
pub fn spawn_unit_sprites_for_new_units(
    mut commands: Commands,
    cfg: Res<TileConfig>,
    tiles_view: Res<TileViewConfig>,
    units_view: Res<UnitViewConfig>,
    teams: Res<Teams>,
    team_colors: Res<TeamColors>,
    // 이미 스프라이트가 없는 유닛 + 트랜스폼도 아직 없는 유닛만 잡아주면 더 안전
    q_units: Query<
        (Entity, &GridPos, &TeamId),
        (With<Unit>, Without<UnitSpriteLink>, Without<Transform>, Without<GlobalTransform>)
    >,
) {
    let base = tiles_view.tile_size(cfg.cell_size);
    let size = Vec2::new(
        base.x * units_view.scale_in_cell,
        base.y * units_view.scale_in_cell,
    );

    for (unit_e, gp, team) in &q_units {
        // ✅ 부모(Unit)를 트랜스폼 계층에 올린다
        commands.entity(unit_e).insert((
            Transform::default(),
            GlobalTransform::default(),
        ));

        let color = team_color(*team, &teams, &team_colors);
        let sprite = Sprite { color, custom_size: Some(size), ..default() };

        let transform = Transform::from_translation(
            cfg.grid_to_world_center(*gp, units_view.z_layer),
        );

        let child = commands
            .spawn(UnitSpriteBundle::new(
                sprite,
                transform,
                UnitVisual { team: *team },
            ))
            .id();

        commands.entity(unit_e)
            .add_child(child)
            .insert(UnitSpriteLink(child));
    }
}

/// 위치/크기/색 동기화
/// - GridPos 변경, TeamId 변경
/// - TileConfig / TileViewConfig / UnitViewConfig 변경
/// - TeamColors 변경
pub fn sync_unit_sprites(
    cfg: Res<TileConfig>,
    tiles_view: Res<TileViewConfig>,
    units_view: Res<UnitViewConfig>,
    team_colors: Res<TeamColors>,
    teams: Res<Teams>,

    q_units: Query<(&GridPos, &TeamId, &UnitSpriteLink), With<Unit>>,
    mut q_sprites: Query<(&mut Transform, &mut Sprite, &mut UnitVisual), With<UnitSprite>>,
) {
    let cfg_changed = cfg.is_changed();
    let tiles_changed = tiles_view.is_changed();
    let units_changed = units_view.is_changed();
    let color_changed = team_colors.is_changed() || teams.is_changed();

    // 사이즈 재계산 필요 여부
    let need_resize = cfg_changed || tiles_changed || units_changed;
    let new_size = if need_resize {
        let base = tiles_view.tile_size(cfg.cell_size);
        Some(Vec2::new(
            base.x * units_view.scale_in_cell,
            base.y * units_view.scale_in_cell,
        ))
    } else {
        None
    };

    for (gp, team, link) in &q_units {
        if let Ok((mut tr, mut sp, mut vis)) = q_sprites.get_mut(link.0) {
            // 위치 싱크(항상)
            tr.translation = cfg.grid_to_world_center(*gp, units_view.z_layer);

            // 크기 싱크(설정 변경 시)
            if let Some(size) = new_size {
                sp.custom_size = Some(size);
            }

            // 색상/팀 메타 싱크(팀 변경 or 팔레트 변경)
            if color_changed || vis.team != *team {
                sp.color = team_color(*team, &teams, &team_colors);
                vis.team = *team;
            }
        }
    }
}
