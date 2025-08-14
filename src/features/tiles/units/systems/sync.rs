//! Unit synchronization systems
//!
//! This module handles health displays and turn refresh systems

use bevy::prelude::*;
use crate::features::tiles::units::components::{Unit, Enemy};

// Health display components
#[derive(Component)]
pub struct AllyHealthText {
    pub owner_entity: Entity,
}

#[derive(Component)]
pub struct EnemyHealthText {
    pub owner_entity: Entity,
}

/// Spawn health text for ally units
pub fn spawn_ally_health_displays(
    mut commands: Commands,
    unit_query: Query<Entity, (With<Unit>, Added<Unit>)>,
    unit_components: Query<&Unit>,
) {
    for entity in unit_query.iter() {
        let Ok(unit) = unit_components.get(entity) else { continue; };
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

/// Spawn health text for enemy units
pub fn spawn_enemy_health_displays(
    mut commands: Commands,
    enemy_query: Query<Entity, (With<Enemy>, Added<Enemy>)>,
    enemy_components: Query<&Enemy>,
) {
    for entity in enemy_query.iter() {
        let Ok(enemy) = enemy_components.get(entity) else { continue; };
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

/// System that resets all units' movement and attack counts when player turn starts
pub fn refresh_units_on_player_turn(
    mut unit_query: Query<(Entity, &mut Unit), With<Unit>>,
) {
    let _unit_count = unit_query.iter().count();
    
    for (_entity, mut unit) in unit_query.iter_mut() {
        let _old_attack_count = unit.attack_count;
        let _old_movement_range = unit.movement_range;
        
        unit.reset_turn_actions();
    }
}