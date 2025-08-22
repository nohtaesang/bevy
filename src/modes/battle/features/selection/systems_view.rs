// =============================================
// src/modes/battle/features/selection/systems_view.rs
// =============================================
use crate::infra::view_core::coords::GridGeometry;
use crate::infra::view_core::z_index::ZLayer;
use crate::modes::battle::features::selection::components::SelectionTileSprite;
use crate::modes::battle::features::selection::resources::{Selected, SelectionHighlightConfig};
use bevy::prelude::*;

/// 선택 하이라이트(타일 1개)를 동기화 (ViewSync)
pub fn sync_selection_highlight(
    mut commands: Commands,
    selected: Res<Selected>,
    grid_geom: Option<Res<GridGeometry>>,
    cfg: Res<SelectionHighlightConfig>,
    mut q: Query<(Entity, &mut Transform, &mut Sprite), With<SelectionTileSprite>>,
    mut last: Local<Option<(u32, u32)>>,
) {
    let Some(grid_geom) = grid_geom else { return; };
    let size = grid_geom.cell * cfg.size_scale;

    let key = selected.tile.map(|g| (g.x, g.y));
    if key == *last && !selected.is_changed() {
        return;
    }
    *last = key;

    match selected.tile {
        Some(g) => {
            let center = grid_geom.grid_center(g);
            if let Ok((_e, mut tr, mut sp)) = q.single_mut() {
                tr.translation =
                    Vec3::new(center.x, center.y, ZLayer::SelectionFx.with_offset(1.0));
                sp.custom_size = Some(size);
                sp.color = cfg.color;
            } else {
                // Bevy 0.14+ 호환: spawn tuple로 교체
                commands.spawn((
                    Transform::from_xyz(center.x, center.y, ZLayer::SelectionFx.with_offset(1.0)),
                    Sprite {
                        custom_size: Some(size),
                        color: cfg.color,
                        ..Default::default()
                    },
                    SelectionTileSprite,
                    Name::new("selection_tile"),
                ));
            }
        }
        None => {
            if let Ok((e, _, _)) = q.single_mut() {
                commands.entity(e).despawn();
            }
        }
    }
}
