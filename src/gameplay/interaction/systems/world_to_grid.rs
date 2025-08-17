use bevy::prelude::*;
use crate::input::{CursorMovedWorld, CursorClickedWorld};
use crate::gameplay::tiles::resources::{BaseTileMap, TileConfig};
use crate::gameplay::interaction::events::{TileHovered, TileClicked, HoverOutside, ClickOutside};

/// 커서 이동(world) → (맵 안) TileHovered / (맵 밖) HoverOutside
pub fn world_cursor_to_grid_hover(
    mut ev_in: EventReader<CursorMovedWorld>,
    mut ev_hover: EventWriter<TileHovered>,
    mut ev_outside: EventWriter<HoverOutside>,
    cfg: Res<TileConfig>,
    map: Option<Res<BaseTileMap>>,
) {
    let Some(map) = map.as_deref() else { ev_in.clear(); return; };

    for e in ev_in.read().copied() {
        let world3 = e.world.extend(0.0);
        if let Some(pos) = cfg.world_to_grid(world3, map) {
            ev_hover.write(TileHovered { pos });
        } else {
            ev_outside.write(HoverOutside { world: e.world });
        }
    }
}

/// 클릭(world) → 좌클릭: (맵 안) TileClicked / (맵 밖) ClickOutside
///                 우클릭: (맵 밖)만 ClickOutside, 맵 안이면 이벤트 없음(취소는 별도 시스템)
pub fn world_click_to_grid_click(
    mut ev_in: EventReader<CursorClickedWorld>,
    mut ev_click: EventWriter<TileClicked>,
    mut ev_outside: EventWriter<ClickOutside>,
    cfg: Res<TileConfig>,
    map: Option<Res<BaseTileMap>>,
) {
    let Some(map) = map.as_deref() else { ev_in.clear(); return; };

    for e in ev_in.read().copied() {
        let world3 = e.world.extend(0.0);
        match e.button {
            MouseButton::Left => {
                if let Some(pos) = cfg.world_to_grid(world3, map) {
                    ev_click.write(TileClicked { pos });
                } else {
                    ev_outside.write(ClickOutside { world: e.world, button: MouseButton::Left });
                }
            }
            MouseButton::Right => {
                // 맵 밖 우클릭만 outside로 알림 (맵 안 우클릭은 별도 취소 시스템이 처리)
                if cfg.world_to_grid(world3, map).is_none() {
                    ev_outside.write(ClickOutside { world: e.world, button: MouseButton::Right });
                }
            }
            _ => {}
        }
    }
}
