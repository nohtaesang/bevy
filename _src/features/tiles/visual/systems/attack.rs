//! Attack overlay systems
//!
//! This module handles attack range visualization

use bevy::prelude::*;
use crate::features::tiles::{
    core::{TileConfig, TileMap, Team, components::TileCoords, tile_to_world_coords},
    selection::SelectionCtx,
    units::bundles::UnitMarker,
    interaction::AttackValidation,
    visual::{
        components::AttackOverlay,
        resources::AttackOverlayState,
    },
};

/// System that cleans up attack overlays when not in Attack state
pub fn cleanup_attack_overlays(
    mut commands: Commands,
    overlay_query: Query<Entity, With<AttackOverlay>>,
) {
    // Despawn all attack overlay entities
    for entity in overlay_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// System that spawns attack overlays based on AttackValidation resource
/// This system reads the valid attacks from AttackValidation and creates overlay entities
pub fn attack_overlay_system(
    mut commands: Commands,
    attack_validation: Res<AttackValidation>,
    tile_config: Res<TileConfig>,
    existing_overlays: Query<Entity, With<AttackOverlay>>,
) {
    // Clean up existing overlays first
    for entity in existing_overlays.iter() {
        commands.entity(entity).despawn();
    }
    
    // Spawn new overlays for each valid attack position
    for &pos in attack_validation.valid_attacks.iter() {
        // Convert tile coordinates to world position using the correct function
        let world_pos_2d = tile_to_world_coords(pos.x, pos.y, &tile_config);
        let world_pos = Vec3::new(
            world_pos_2d.x,
            world_pos_2d.y,
            1.0, // Above tile but below UI
        );
        
        // Spawn attack overlay entity
        commands.spawn((
            Transform::from_translation(world_pos),
            GlobalTransform::default(),
            Sprite {
                color: Color::srgba(1.0, 0.0, 0.0, 0.3), // Semi-transparent red
                custom_size: Some(Vec2::splat(tile_config.tile_size * 0.9)),
                ..default()
            },
            AttackOverlay { tile_pos: pos },
            TileCoords { x: pos.x, y: pos.y },
        ));
    }
}