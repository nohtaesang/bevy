// src/gameplay/interaction/systems/hotkeys.rs
use bevy::prelude::*;
use crate::input::KeyJustPressed;
use crate::gameplay::interaction::{
    resources::SelectionCtx,
    state::PlayerIntent,
    events::PlayerIntentChanged,
};

/// 핫키 → 의도 전환 (실행/검증은 다른 시스템에서 처리)
pub fn handle_hotkeys_to_intent(
    mut ev_keys: EventReader<KeyJustPressed>,
    mut sel: ResMut<SelectionCtx>,
    mut ev_changed: EventWriter<PlayerIntentChanged>,
) {
    // 기본은 현재 의도 유지, 마지막으로 눌린 키를 우선
    let mut target = sel.intent;

    for KeyJustPressed(key) in ev_keys.read().copied() {
        match key {
            // 이동/공격 (넘패드 포함)
            KeyCode::Digit1 | KeyCode::Numpad1 => target = PlayerIntent::Move,
            KeyCode::Digit2 | KeyCode::Numpad2 => target = PlayerIntent::Attack,

            // 취소
            KeyCode::Escape => {
                target = PlayerIntent::Idle;
                // 선택도 함께 해제하고 싶다면:
                // let (_t, _u) = sel.clear_selection();
            }

            _ => {}
        }
    }

    if target != sel.intent {
        let (prev, new_intent) = sel.set_intent(target);
        ev_changed.write(PlayerIntentChanged { prev, new_intent });
    }
}
