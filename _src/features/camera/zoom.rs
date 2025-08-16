use bevy::prelude::*;

/// Camera zoom system: Mouse wheel
pub fn camera_zoom(
    mut scroll_events: EventReader<bevy::input::mouse::MouseWheel>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    // Mouse wheel zoom (using transform scale)
    for scroll in scroll_events.read() {
        let zoom_speed = 0.1;
        let zoom_factor = 1.0 - (scroll.y * zoom_speed);
        
        // Apply zoom with limits using transform scale
        let current_scale = camera_transform.scale.x;
        let new_scale = (current_scale * zoom_factor).clamp(0.1, 5.0);
        camera_transform.scale = Vec3::splat(new_scale);
    }
}