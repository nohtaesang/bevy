use super::resources::{HoverHighlightConfig, HoverTile};
use crate::infra::view_core::coords::GridGeometry;
use crate::infra::view_core::z_index::ZLayer;
use bevy::prelude::*;

#[derive(Component)]
pub struct HoverTileSprite;

pub fn sync_hover_highlight(
    mut commands: Commands,
    hover: Res<HoverTile>,
    grid_geom: Option<Res<GridGeometry>>,
    cfg: Res<HoverHighlightConfig>,
    mut q: Query<(Entity, &mut Transform, &mut Sprite), With<HoverTileSprite>>,
    mut last: Local<Option<(u32, u32)>>, // 로컬로 이전 상태만 기억
) {
    let Some(grid_geom) = grid_geom else { return; };
    let size = grid_geom.cell * cfg.size_scale;

    // 변경 없으면 스킵
    let curr_key = hover.grid.map(|g| (g.x, g.y));
    if curr_key == *last && !hover.is_changed() {
        return;
    }
    *last = curr_key;

    match hover.grid {
        Some(g) => {
            let center = grid_geom.grid_center(g);
            if let Ok((_e, mut tr, mut sp)) = q.single_mut() {
                tr.translation = Vec3::new(center.x, center.y, ZLayer::SelectionFx.z());
                sp.custom_size = Some(size);
                sp.color = cfg.color;
            } else {
                // ⚠️ Bevy 0.14+ 호환: 기본 스폰 후 insert로 교체
                commands.spawn((
                    Transform::from_xyz(center.x, center.y, ZLayer::SelectionFx.z()),
                    Sprite {
                        custom_size: Some(size),
                        color: cfg.color,
                        ..Default::default()
                    },
                    HoverTileSprite,
                    Name::new("hover_tile"),
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
