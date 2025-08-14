//! Movement overlay systems
//!
//! This module handles movement range visualization

use bevy::prelude::*;
use crate::features::tiles::{
    core::{TileConfig, TileMap, Team, components::TileCoords, tile_to_world_coords},
    selection::SelectionCtx,
    units::bundles::UnitMarker,
    interaction::MovementValidation,
    visual::{
        components::MovementOverlay,
        resources::MovementOverlayState,
    },
};

/// System that cleans up movement overlays when not in Move state
pub fn cleanup_movement_overlays(
    mut commands: Commands,
    overlay_query: Query<Entity, With<MovementOverlay>>,
) {
    // Despawn all movement overlay entities
    for entity in overlay_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// System that spawns movement overlays based on MovementValidation resource
/// This system reads the valid moves from MovementValidation and creates overlay entities
pub fn movement_overlay_system(
    mut commands: Commands,
    movement_validation: Res<MovementValidation>,
    tile_config: Res<TileConfig>,
    existing_overlays: Query<Entity, With<MovementOverlay>>,
) {
    // Clean up existing overlays first
    for entity in existing_overlays.iter() {
        commands.entity(entity).despawn();
    }
    
    // Spawn new overlays for each valid move position
    for &pos in movement_validation.valid_moves.iter() {
        // Convert tile coordinates to world position using the correct function
        let world_pos_2d = tile_to_world_coords(pos.x, pos.y, &tile_config);
        let world_pos = Vec3::new(
            world_pos_2d.x,
            world_pos_2d.y,
            1.0, // Above tile but below UI
        );
        
        // Spawn movement overlay entity
        commands.spawn((
            Transform::from_translation(world_pos),
            GlobalTransform::default(),
            Sprite {
                color: Color::srgba(0.0, 1.0, 0.0, 0.3), // Semi-transparent green
                custom_size: Some(Vec2::splat(tile_config.tile_size * 0.9)),
                ..default()
            },
            MovementOverlay { tile_pos: pos },
            TileCoords { x: pos.x, y: pos.y },
        ));
    }
}