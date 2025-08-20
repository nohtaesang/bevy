// ===============================
// infra/view_core/camera/apply.rs
// ===============================
use bevy::prelude::*;
use bevy::render::camera::Projection;
use bevy::window::PrimaryWindow;

use super::types::{CameraIntent, CameraSettings, MainCamera};

/// Intent를 한 번에 적용하고 Intent를 리셋
pub fn camera_apply_intent(
    mut q_cam: Query<
        (&Camera, &GlobalTransform, &mut Transform, &mut Projection),
        With<MainCamera>,
    >,
    settings: Res<CameraSettings>,
    mut intent: ResMut<CameraIntent>,
    q_win: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok((camera, cam_gt, mut tf, mut proj)) = q_cam.get_single_mut() else {
        return;
    };

    // 1) 키보드 팬(world)
    if intent.pan_world != Vec2::ZERO {
        let delta = tf.rotation * intent.pan_world.extend(0.0);
        let z = tf.translation.z;
        tf.translation += delta;
        tf.translation.z = z;
    }

    // 2) 드래그 팬(screen→world)
    if intent.drag_pixels != Vec2::ZERO {
        // ⬇️ 1px → (scale * multiplier) world 로 단순화
        let world_per_px = match &*proj {
            Projection::Orthographic(o) => o.scale,
            _ => 1.0,
        } * settings.pan_drag_multiplier;

        // 화면 우(+)/상(+) 드래그 = 맵은 반대로 (X 반전 유지)
        let screen = Vec2::new(-intent.drag_pixels.x, intent.drag_pixels.y) * world_per_px;

        // 카메라 회전 보정
        let world = tf.rotation * screen.extend(0.0);

        let z = tf.translation.z;
        tf.translation += world;
        tf.translation.z = z;
    }

    // 3) 회전
    if intent.rotate != 0.0 {
        tf.rotate_z(intent.rotate);
    }

    // 4) 줌 (커서 고정)
    if intent.zoom_steps != 0.0 {
        // 커서 아래 월드 좌표(before)
        let before_world: Option<Vec2> = q_win
            .single()
            .ok()
            .and_then(|w| w.cursor_position())
            .and_then(|cursor| camera.viewport_to_world_2d(cam_gt, cursor).ok());

        // 배율 변경: steps>0 → 줌인 (scale 감소)
        if let Projection::Orthographic(o) = &mut *proj {
            let factor = (1.0 - intent.zoom_steps).clamp(0.2, 5.0);
            o.scale = (o.scale * factor).clamp(settings.zoom_min, settings.zoom_max);
        }

        // 커서 고정 보정
        if let Ok(win) = q_win.single() {
            if let Some(cursor) = win.cursor_position() {
                if let Ok(after_world) = camera.viewport_to_world_2d(cam_gt, cursor) {
                    if let Some(before) = before_world {
                        let delta = before - after_world;
                        let z = tf.translation.z;
                        tf.translation.x += delta.x;
                        tf.translation.y += delta.y;
                        tf.translation.z = z;
                    }
                }
            }
        }
    }

    // Intent reset
    *intent = CameraIntent::default();
}
