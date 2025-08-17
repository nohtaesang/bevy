use bevy::prelude::*;

use crate::{
    gameplay::{
        interaction::resources::SelectionCtx,
        tiles::resources::TileConfig,
    },
    view::tiles::resources::TileViewConfig,
};

use super::{
    components::{HoverTileOverlay, SelectedTileOverlay, OverlaySpriteBundle},
    resources::TileOverlayConfig,
};

/// Hover/Selected 오버레이 스프라이트를 각각 1장씩 생성(기본 Hidden)
pub fn spawn_tile_overlays_once(
    mut commands: Commands,
    cfg: Res<TileOverlayConfig>,
    q_hover: Query<Entity, With<HoverTileOverlay>>,
    q_selected: Query<Entity, With<SelectedTileOverlay>>,
) {
    // Hover overlay 없으면 생성
    if q_hover.is_empty() {
        commands.spawn((
            OverlaySpriteBundle::new_hidden(cfg.hover_color),
            HoverTileOverlay,
        ));
    }

    // Selected overlay 없으면 생성
    if q_selected.is_empty() {
        commands.spawn((
            OverlaySpriteBundle::new_hidden(cfg.selected_color),
            SelectedTileOverlay,
        ));
    }
}

/// Hover 오버레이: SelectionCtx.hover_tile에 맞춰 위치/크기/가시성 동기화
pub fn sync_hover_overlay(
    ctx: Res<SelectionCtx>,
    tile_cfg: Res<TileConfig>,
    tile_view: Res<TileViewConfig>,
    overlay_cfg: Res<TileOverlayConfig>,
    mut q: Query<(&mut Transform, &mut Sprite, &mut Visibility), With<HoverTileOverlay>>,
) {
    // 성능 게이팅: 관련 리소스 변화 없으면 스킵
    if !(ctx.is_changed() || tile_cfg.is_changed() || tile_view.is_changed() || overlay_cfg.is_changed()) {
        return;
    }

    let Ok((mut tr, mut sp, mut vis)) = q.get_single_mut() else { return; };

    if let Some(gp) = ctx.hover_tile {
        // 위치(Z는 cfg.z_hover 사용)
        tr.translation = tile_cfg.grid_to_world_center(gp, overlay_cfg.z_hover);

        // 타일 실크기 * 스케일
        let base = tile_view.tile_size(tile_cfg.cell_size);
        sp.custom_size = Some(base * overlay_cfg.scale_in_cell);

        *vis = Visibility::Visible;
    } else {
        *vis = Visibility::Hidden;
    }
}

/// Selected 오버레이: SelectionCtx.selected_tile에 맞춰 위치/크기/가시성 동기화
pub fn sync_selected_overlay(
    ctx: Res<SelectionCtx>,
    tile_cfg: Res<TileConfig>,
    tile_view: Res<TileViewConfig>,
    overlay_cfg: Res<TileOverlayConfig>,
    mut q: Query<(&mut Transform, &mut Sprite, &mut Visibility), With<SelectedTileOverlay>>,
) {
    // 성능 게이팅: 관련 리소스 변화 없으면 스킵
    if !(ctx.is_changed() || tile_cfg.is_changed() || tile_view.is_changed() || overlay_cfg.is_changed()) {
        return;
    }

    let Ok((mut tr, mut sp, mut vis)) = q.get_single_mut() else { return; };

    if let Some(gp) = ctx.selected_tile {
        // 위치(Z는 cfg.z_selected 사용)
        tr.translation = tile_cfg.grid_to_world_center(gp, overlay_cfg.z_selected);

        // 타일 실크기 * 스케일
        let base = tile_view.tile_size(tile_cfg.cell_size);
        sp.custom_size = Some(base * overlay_cfg.scale_in_cell);

        *vis = Visibility::Visible;
    } else {
        *vis = Visibility::Hidden;
    }
}
