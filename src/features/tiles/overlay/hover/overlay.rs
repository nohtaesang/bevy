//! Hover overlay component and creation functions

use bevy::prelude::*;

#[derive(Component)]
pub struct HoverOverlay;

pub fn create_hover_overlay_sprite(commands: &mut Commands, tile_size: f32) {
    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.3), // Semi-transparent white
            custom_size: Some(Vec2::new(tile_size, tile_size)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)), // Z-index 0 (below everything)
        Visibility::Hidden,
        HoverOverlay,
    ));
}