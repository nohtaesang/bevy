use bevy::prelude::*;
use crate::input::CursorClickedWorld;
use crate::gameplay::interaction::{
    resources::SelectionCtx,
    state::PlayerIntent,
    events::{PlayerIntentChanged, SelectionChanged},
};

/// 우클릭 취소 규칙:
/// - Move/Attack 중이면 의도만 Idle로 전환(선택 유지)
/// - Idle이면 선택이 있으면 해제
pub fn cancel_on_right_click(
    mut ev_click_world: EventReader<CursorClickedWorld>,
    mut sel: ResMut<SelectionCtx>,
    mut ev_intent: EventWriter<PlayerIntentChanged>,
    mut ev_sel: EventWriter<SelectionChanged>,
) {
    // 이번 프레임에 우클릭이 1회라도 있었는지 확인 (좌표/안밖 무관)
    let mut right_clicked = false;
    for e in ev_click_world.read() {
        if e.button == MouseButton::Right {
            right_clicked = true;
        }
    }
    if !right_clicked {
        return;
    }

    match sel.intent {
        PlayerIntent::Move | PlayerIntent::Attack => {
            let (prev, new_intent) = sel.set_intent(PlayerIntent::Idle);
            ev_intent.write(PlayerIntentChanged { prev, new_intent });
        }
        PlayerIntent::Idle => {
            if sel.selected_tile.is_some() || sel.selected_unit.is_some() {
                let (t, u) = sel.clear_selection();
                if t || u {
                    ev_sel.write(SelectionChanged { tile_changed: t, unit_changed: u });
                }
            }
        }
    }
}
