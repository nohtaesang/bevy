use bevy::prelude::*;
use crate::gameplay::interaction::{
    resources::SelectionCtx,
    state::PlayerIntent,
    events::{TileClicked, ClickOutside, SelectionChanged, CommandRequested},
};
use crate::gameplay::tiles::resources::GridIndex;
use crate::gameplay::units::components::Unit;

pub fn handle_clicks_by_intent(
    mut ev_click: EventReader<TileClicked>,
    mut ev_outside: EventReader<ClickOutside>,
    mut sel: ResMut<SelectionCtx>,
    mut ev_sel_changed: EventWriter<SelectionChanged>,
    mut ev_cmd: EventWriter<CommandRequested>,
    // ⬇️ 추가: 그리드 점유표, 그리고 유닛 검증용 쿼리
    grid: Res<GridIndex>,
    q_units: Query<(), With<Unit>>,
) {
    // 1) 맵 안 클릭
    for TileClicked { pos } in ev_click.read().copied() {
        match sel.intent {
            PlayerIntent::Idle => {
                // ⬇️ 새로 추가: 클릭한 칸에 유닛이 있으면 Some(Entity)
                let unit_at_tile = grid
                    .get_pos(pos)            // Option<Entity>
                    .filter(|&e| q_units.get(e).is_ok()); // 진짜 Unit 컴포넌트가 있는지 확인

                let (tile_changed, unit_changed) = sel.set_selection(Some(pos), unit_at_tile);
                if tile_changed || unit_changed {
                    ev_sel_changed.write(SelectionChanged { tile_changed, unit_changed });
                }
            }
            PlayerIntent::Move => {
                ev_cmd.write(CommandRequested::MoveTo { to: pos });
            }
            PlayerIntent::Attack => {
                ev_cmd.write(CommandRequested::AttackTile { at: pos });
            }
        }
    }

    // 2) 맵 밖 클릭 → (Idle일 때만) 선택 해제
    if !ev_outside.is_empty() {
        if sel.intent == PlayerIntent::Idle {
            let (tile_changed, unit_changed) = sel.clear_selection();
            if tile_changed || unit_changed {
                ev_sel_changed.write(SelectionChanged { tile_changed, unit_changed });
            }
        }
        ev_outside.clear();
    }
}
