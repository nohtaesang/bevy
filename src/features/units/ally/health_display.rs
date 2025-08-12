//! Health display system for ally units

use bevy::prelude::*;
use super::Unit;

#[derive(Component)]
pub struct AllyHealthText {
    pub owner_entity: Entity,
}

/// Spawn health text for ally units
pub fn spawn_ally_health_displays(
    mut commands: Commands,
    unit_query: Query<(Entity, &Unit), (With<Unit>, Without<AllyHealthText>)>,
) {
    for (entity, unit) in unit_query.iter() {
        let health_text = format!("{}/{}", unit.health, unit.max_health);
        let health_ratio = unit.health as f32 / unit.max_health as f32;
        let color = if health_ratio > 0.6 {
            Color::srgb(0.0, 1.0, 0.0) // Green
        } else if health_ratio > 0.3 {
            Color::srgb(1.0, 1.0, 0.0) // Yellow
        } else {
            Color::srgb(1.0, 0.0, 0.0) // Red
        };
        
        let text_entity = commands.spawn((
            Text2d::new(health_text),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(color),
            Transform::from_translation(Vec3::new(0.0, 30.0, 1.0)),
            AllyHealthText { owner_entity: entity },
        )).id();
        
        commands.entity(entity).add_child(text_entity);
    }
}

/// Update ally health text when health changes
pub fn update_ally_health_displays(
    mut health_text_query: Query<(&mut Text2d, &mut TextColor, &AllyHealthText)>,
    changed_units: Query<(Entity, &Unit), Changed<Unit>>,
) {
    for (unit_entity, unit) in changed_units.iter() {
        for (mut text, mut text_color, health_text) in health_text_query.iter_mut() {
            if health_text.owner_entity == unit_entity {
                **text = format!("{}/{}", unit.health, unit.max_health);
                
                // Color based on health percentage
                let health_ratio = unit.health as f32 / unit.max_health as f32;
                text_color.0 = if health_ratio > 0.6 {
                    Color::srgb(0.0, 1.0, 0.0) // Green
                } else if health_ratio > 0.3 {
                    Color::srgb(1.0, 1.0, 0.0) // Yellow
                } else {
                    Color::srgb(1.0, 0.0, 0.0) // Red
                };
                break;
            }
        }
    }
}

/// Clean up ally health text when units are removed
pub fn cleanup_ally_health_displays(
    mut commands: Commands,
    health_text_query: Query<(Entity, &AllyHealthText)>,
    unit_query: Query<Entity, With<Unit>>,
) {
    for (text_entity, health_text) in health_text_query.iter() {
        if unit_query.get(health_text.owner_entity).is_err() {
            commands.entity(text_entity).despawn();
        }
    }
}