//! Selected overlay component and creation functions

use bevy::prelude::*;

#[derive(Component)]
pub struct SelectedOverlay;

pub fn create_selected_overlay_sprite(commands: &mut Commands, tile_size: f32) {
    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 1.0, 0.0, 0.5), // Semi-transparent yellow
            custom_size: Some(Vec2::new(tile_size, tile_size)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)), // Z-index 2 (above hover)
        Visibility::Hidden,
        SelectedOverlay,
    ));
}