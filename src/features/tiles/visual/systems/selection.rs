//! Selection overlay systems
//!
//! This module handles selected tile visual effects

use bevy::prelude::*;
use crate::features::tiles::{
    core::{TileConfig, tile_to_world_coords},
    selection::SelectionCtx,
    visual::{components::{SelectedOverlay, Overlay}, z},
};

/// System that renders selection overlay based on SelectionCtx resource
/// Converts tile coordinates to world position using current TileConfig
pub fn render_selection_overlay(
    selection_ctx: Res<SelectionCtx>,
    tile_config: Res<TileConfig>,
    mut overlay_query: Query<(&mut Transform, &mut Visibility), With<SelectedOverlay>>,
) {
    let Ok((mut transform, mut visibility)) = overlay_query.single_mut() else {
        return;
    };
    
    // Update overlay based on SelectionCtx resource state
    if let Some(tile_pos) = selection_ctx.tile {
        // Convert tile coordinates to world position using current TileConfig
        let world_pos = tile_to_world_coords(tile_pos.x, tile_pos.y, &tile_config);
        
        // Show overlay at calculated world position
        transform.translation.x = world_pos.x;
        transform.translation.y = world_pos.y;
        *visibility = Visibility::Visible;
    } else {
        // Hide overlay when nothing is selected
        *visibility = Visibility::Hidden;
    }
}

/// Spawn the selection overlay entity on game start
pub fn spawn_selection_overlay(
    mut commands: Commands,
    tile_config: Res<TileConfig>,
    existing_overlay: Query<Entity, With<SelectedOverlay>>,
) {
    // Skip if overlay already exists
    if !existing_overlay.is_empty() {
        return;
    }
    
    spawn_selection_overlay_internal(&mut commands, &tile_config);
}

/// Ensure selection overlay exists - creates one if missing
pub fn ensure_selection_overlay(
    mut commands: Commands,
    tile_config: Res<TileConfig>,
    existing_overlay: Query<Entity, With<SelectedOverlay>>,
    selection_ctx: Res<SelectionCtx>,
) {
    // Only check if we have something selected but no overlay exists
    if selection_ctx.tile.is_some() && existing_overlay.is_empty() {
        warn!("Selection overlay missing while tile selected - creating new overlay");
        spawn_selection_overlay_internal(&mut commands, &tile_config);
    }
}

/// Internal function to spawn selection overlay entity
fn spawn_selection_overlay_internal(
    commands: &mut Commands,
    tile_config: &TileConfig,
) {
    // Spawn selection overlay entity (initially hidden)
    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 1.0, 0.0, 0.5), // Semi-transparent yellow
            custom_size: Some(Vec2::new(tile_config.tile_size, tile_config.tile_size)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, z::SELECTION), // Selection overlay layer
        Visibility::Hidden, // Start hidden
        SelectedOverlay,
        Overlay,
        Name::new("Selection Overlay"),
    ));
    
    info!("Spawned selection overlay entity");
}