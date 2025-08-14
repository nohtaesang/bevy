//! Hover overlay systems
//!
//! This module handles tile hover visual effects

use bevy::prelude::*;
use crate::features::tiles::{
    core::TileConfig,
    visual::{components::{HoverOverlay, HoverMarker}, z},
    // Only import HoverTile from interaction prelude (read-only)
    interaction::prelude::HoverTile,
};

/// System that updates hover overlay visual based on HoverTile resource
/// Converts tile coordinates to world position using current TileConfig
pub fn render_hover_overlay(
    hover_tile: Res<HoverTile>,
    tile_config: Res<TileConfig>,
    mut overlay_query: Query<(&mut Transform, &mut Visibility), With<HoverOverlay>>,
) {
    let Ok((mut transform, mut visibility)) = overlay_query.single_mut() else {
        return;
    };
    
    // Update overlay based on HoverTile resource state
    if let Some(tile_pos) = hover_tile.tile_pos {
        // Convert tile coordinates to world position using current TileConfig
        let world_pos = crate::features::tiles::core::tile_to_world_coords(
            tile_pos.x, 
            tile_pos.y, 
            &tile_config
        );
        
        // Show overlay at calculated world position
        transform.translation.x = world_pos.x;
        transform.translation.y = world_pos.y;
        *visibility = Visibility::Visible;
    } else {
        // Hide overlay when not hovering
        *visibility = Visibility::Hidden;
    }
}

/// Spawn the hover overlay entity on game start
pub fn spawn_hover_overlay(
    mut commands: Commands,
    tile_config: Res<TileConfig>,
    existing_overlay: Query<Entity, With<HoverOverlay>>,
) {
    // Skip if overlay already exists
    if !existing_overlay.is_empty() {
        return;
    }
    
    spawn_hover_overlay_internal(&mut commands, &tile_config);
}

/// Ensure hover overlay exists - creates one if missing
pub fn ensure_hover_overlay(
    mut commands: Commands,
    tile_config: Res<TileConfig>,
    existing_overlay: Query<Entity, With<HoverOverlay>>,
    hover_tile: Res<HoverTile>,
) {
    // Only check if we're trying to hover but no overlay exists
    if hover_tile.is_hovering() && existing_overlay.is_empty() {
        warn!("Hover overlay missing while hovering - creating new overlay");
        spawn_hover_overlay_internal(&mut commands, &tile_config);
    }
}

/// Internal function to spawn hover overlay entity
fn spawn_hover_overlay_internal(
    commands: &mut Commands,
    tile_config: &TileConfig,
) {
    // Spawn hover overlay entity (initially hidden)
    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.3), // Semi-transparent white
            custom_size: Some(Vec2::new(tile_config.tile_size, tile_config.tile_size)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, z::HOVER), // Hover overlay layer
        Visibility::Hidden, // Start hidden
        HoverOverlay,
        HoverMarker,
        Name::new("Hover Overlay"),
    ));
    
    info!("Spawned hover overlay entity");
}