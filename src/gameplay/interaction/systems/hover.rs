// src/gameplay/interaction/systems/hover.rs
use bevy::prelude::*;
use crate::gameplay::interaction::{
    events::{TileHovered, HoverOutside},
    resources::SelectionCtx,
};
use crate::gameplay::tiles::components::GridPos;

/// Hover 반영:
/// - 같은 프레임에 여러 Hover가 오면 마지막 것만 사용
/// - HoverOutside가 있으면 Hover를 해제(우선권)
pub fn apply_hover_to_ctx(
    mut ev_hover: EventReader<TileHovered>,
    mut ev_outside: EventReader<HoverOutside>,
    mut sel: ResMut<SelectionCtx>,
) {
    // 1) 바깥 이벤트가 있으면 우선 해제
    if !ev_outside.is_empty() {
        sel.set_hover(None);
        ev_outside.clear(); // 소비
        return;
    }

    // 2) 맵 안 Hover가 여러 번 오더라도 마지막 값만 반영
    let mut last: Option<GridPos> = None;
    for TileHovered { pos } in ev_hover.read().copied() {
        last = Some(pos);
    }
    if let Some(p) = last {
        sel.set_hover(Some(p));
    }
}
