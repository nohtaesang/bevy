use bevy::prelude::*;
use crate::gameplay::interaction::{
    resources::SelectionCtx,
    state::PlayerIntent,
    events::{TileClicked, ClickOutside, SelectionChanged, CommandRequested},
};

pub fn handle_clicks_by_intent(
    mut ev_click: EventReader<TileClicked>,
    mut ev_outside: EventReader<ClickOutside>,
    mut sel: ResMut<SelectionCtx>,
    mut ev_sel_changed: EventWriter<SelectionChanged>,
    mut ev_cmd: EventWriter<CommandRequested>,
) {
    // 1) 맵 안 클릭
    for TileClicked { pos } in ev_click.read().copied() {
        match sel.intent {
            PlayerIntent::Idle => {
                // ⚠️ 필드를 먼저 로컬로 복사한 뒤 &mut self 메서드 호출
                let prev_unit = sel.selected_unit;
                let (tile_changed, unit_changed) = sel.set_selection(Some(pos), prev_unit);
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
