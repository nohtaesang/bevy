//! Health display system for enemy units

use bevy::prelude::*;
use super::Enemy;

#[derive(Component)]
pub struct EnemyHealthText {
    pub owner_entity: Entity,
}

/// Spawn health text for enemy units
pub fn spawn_enemy_health_displays(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Enemy), (With<Enemy>, Without<EnemyHealthText>)>,
) {
    for (entity, enemy) in enemy_query.iter() {
        let health_text = format!("{}/{}", enemy.health, enemy.max_health);
        let health_ratio = enemy.health as f32 / enemy.max_health as f32;
        let color = if health_ratio > 0.6 {
            Color::srgb(1.0, 0.5, 0.5) // Light red
        } else if health_ratio > 0.3 {
            Color::srgb(1.0, 0.0, 0.0) // Red
        } else {
            Color::srgb(0.8, 0.0, 0.0) // Dark red
        };
        
        let text_entity = commands.spawn((
            Text2d::new(health_text),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(color),
            Transform::from_translation(Vec3::new(0.0, 30.0, 1.0)),
            EnemyHealthText { owner_entity: entity },
        )).id();
        
        commands.entity(entity).add_child(text_entity);
    }
}

/// Update enemy health text when health changes
pub fn update_enemy_health_displays(
    mut health_text_query: Query<(&mut Text2d, &mut TextColor, &EnemyHealthText)>,
    changed_enemies: Query<(Entity, &Enemy), Changed<Enemy>>,
) {
    for (enemy_entity, enemy) in changed_enemies.iter() {
        for (mut text, mut text_color, health_text) in health_text_query.iter_mut() {
            if health_text.owner_entity == enemy_entity {
                **text = format!("{}/{}", enemy.health, enemy.max_health);
                
                // Color based on health percentage
                let health_ratio = enemy.health as f32 / enemy.max_health as f32;
                text_color.0 = if health_ratio > 0.6 {
                    Color::srgb(1.0, 0.5, 0.5) // Light red
                } else if health_ratio > 0.3 {
                    Color::srgb(1.0, 0.0, 0.0) // Red
                } else {
                    Color::srgb(0.8, 0.0, 0.0) // Dark red
                };
                break;
            }
        }
    }
}

/// Clean up enemy health text when enemies are removed
pub fn cleanup_enemy_health_displays(
    mut commands: Commands,
    health_text_query: Query<(Entity, &EnemyHealthText)>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for (text_entity, health_text) in health_text_query.iter() {
        if enemy_query.get(health_text.owner_entity).is_err() {
            commands.entity(text_entity).despawn();
        }
    }
}