use bevy::prelude::*;

use super::{
    resources::CursorWorldPos,
    events::{CursorMovedWorld, CursorClickedWorld, KeyJustPressed},
};

// 네가 만든 메인 카메라 마커를 불러온다.
// (경로가 다르면 여기만 맞춰주면 됨)
use crate::view::camera::movement::MainCamera;

/// 화면 커서 → 월드 좌표로 변환하고, 값이 바뀌면 CursorMovedWorld 이벤트 발행
pub fn update_cursor_world_pos(
    windows: Query<&Window>,
    cam_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor: ResMut<CursorWorldPos>,
    mut ev_moved: EventWriter<CursorMovedWorld>,
) {
    // 단일 윈도우 전제 (멀티 윈도우면 구분 로직 추가)
    let window = match windows.single() {
        Ok(w) => w,
        Err(_) => {
            cursor.screen = None;
            cursor.world = None;
            return;
        }
    };

    let screen = window.cursor_position();
    cursor.screen = screen;

    if let (Some(screen_pos), Ok((camera, gt))) = (screen, cam_q.single()) {
        // Bevy 0.16: Result → Option으로 변환
        let world_opt = camera.viewport_to_world_2d(gt, screen_pos).ok();
        // 이벤트는 "좌표가 바뀐 경우"에만 내보낸다
        if world_opt != cursor.world {
            cursor.world = world_opt;
            if let Some(world) = world_opt {
                ev_moved.send(CursorMovedWorld { world, screen: screen_pos });
            }
        }
    } else {
        cursor.world = None;
    }
}

/// 클릭 시 현재 월드 좌표를 포함해 CursorClickedWorld 발행
pub fn emit_clicks_world(
    mouse: Res<ButtonInput<MouseButton>>,
    cursor: Res<CursorWorldPos>,
    mut ev_click: EventWriter<CursorClickedWorld>,
) {
    // 월드 좌표가 유효할 때만 클릭 이벤트 발행
    if let Some(world) = cursor.world {
        if mouse.just_pressed(MouseButton::Left) {
            ev_click.send(CursorClickedWorld { world, button: MouseButton::Left });
        }
        if mouse.just_pressed(MouseButton::Right) {
            ev_click.send(CursorClickedWorld { world, button: MouseButton::Right });
        }
        if mouse.just_pressed(MouseButton::Middle) {
            ev_click.send(CursorClickedWorld { world, button: MouseButton::Middle });
        }
    }
}

pub fn emit_key_just_pressed(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_key: EventWriter<KeyJustPressed>,
) {
    // 전역 키맵: 여기선 몇 개만 예시. 필요 키를 추가/삭제하세요.
    const WATCH_KEYS: &[KeyCode] = &[
        KeyCode::Digit1, KeyCode::Digit2, KeyCode::Escape,
        KeyCode::Space,  KeyCode::KeyM,   KeyCode::KeyA,
    ];

    for &k in WATCH_KEYS {
        if keys.just_pressed(k) {
            ev_key.send(KeyJustPressed(k));
        }
    }
}