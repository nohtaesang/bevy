//! Unit synchronization systems
//!
//! This module handles health displays and turn refresh systems

use bevy::prelude::*;
use crate::features::tiles::{
    core::Team,
    units::bundles::UnitMarker,
};

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
    mut _commands: Commands,
    unit_query: Query<Entity, (With<UnitMarker>, Added<UnitMarker>)>,
    _unit_components: Query<&Team, With<UnitMarker>>,
) {
    for _entity in unit_query.iter() {
        // TODO: Get health from CurrentStats component
        // TODO: Implement health display UI with proper stats system
    }
}

/// Update ally health text when health changes  
pub fn update_ally_health_displays(
    mut _health_text_query: Query<(&mut Text2d, &mut TextColor, &AllyHealthText)>,
    _changed_units: Query<Entity, (With<UnitMarker>, Changed<UnitMarker>)>,
) {
    // TODO: Implement health display updates with CurrentStats component
}

/// Clean up ally health text when units are removed
pub fn cleanup_ally_health_displays(
    mut commands: Commands,
    health_text_query: Query<(Entity, &AllyHealthText)>,
    _unit_query: Query<Entity, With<UnitMarker>>,
) {
    for (text_entity, _health_text) in health_text_query.iter() {
        // TODO: Check if unit still exists and clean up accordingly
        commands.entity(text_entity).despawn();
    }
}

/// Spawn health text for enemy units
pub fn spawn_enemy_health_displays(
    mut commands: Commands,
    _enemy_query: Query<Entity, (With<UnitMarker>, With<Team>)>,
    _enemy_components: Query<&Team, With<UnitMarker>>,
) {
    // TODO: Implement enemy health display with CurrentStats component
}

/// Update enemy health text when health changes
pub fn update_enemy_health_displays(
    mut health_text_query: Query<(&mut Text2d, &mut TextColor, &EnemyHealthText)>,
    _changed_enemies: Query<Entity, (With<UnitMarker>, With<Team>)>,
) {
    // TODO: Implement enemy health display updates with CurrentStats component
}

/// Clean up enemy health text when enemies are removed
pub fn cleanup_enemy_health_displays(
    mut commands: Commands,
    health_text_query: Query<(Entity, &EnemyHealthText)>,
    _enemy_query: Query<Entity, With<UnitMarker>>,
) {
    for (text_entity, _health_text) in health_text_query.iter() {
        // TODO: Check if enemy still exists and clean up accordingly
        commands.entity(text_entity).despawn();
    }
}

/// System that resets all units' movement and attack counts when player turn starts
pub fn refresh_units_on_player_turn(
    mut _unit_query: Query<Entity, With<UnitMarker>>,
) {
    // TODO: Implement turn refresh with CurrentStats component
    // TODO: Reset movement_range and attack_count to base values from BaseStats
}