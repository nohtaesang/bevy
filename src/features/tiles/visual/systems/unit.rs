//! Unit visual rendering system
//!
//! Handles spawning and syncing visual sprites for units using Parent/Child hierarchy

use bevy::prelude::*;
use crate::features::tiles::{
    core::{TileConfig, tile_to_world_coords, UnitSpawned, Team, components::TileCoords},
    units::bundles::UnitMarker,
    visual::{components::UnitVisual, z},
};

/// Spawn visual sprite as child of logic unit entity
pub fn spawn_unit_visual_on_spawned(
    mut commands: Commands,
    cfg: Res<TileConfig>,
    mut ev: EventReader<UnitSpawned>,
) {
    for e in ev.read() {
        let wp = tile_to_world_coords(e.position.x, e.position.y, &cfg);
        let size = cfg.tile_size - cfg.tile_spacing;

        // Parent Transform with world position
        commands.entity(e.entity).insert((
            Transform::from_xyz(wp.x, wp.y, 0.0),  // Parent at world position with z=0
            GlobalTransform::default(),
        ));

        // Spawn visual sprite as child with local coordinates
        commands.entity(e.entity).with_children(|parent| {
            parent.spawn((
                Sprite {
                    color: match e.team {
                        Team::Player => Color::srgb(0.2, 0.8, 0.2),
                        Team::Enemy  => Color::srgb(0.85, 0.25, 0.25),
                    },
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, z::UNIT),  // Child at local origin with proper z-layer
                UnitVisual,
                Name::new(format!("UnitVisual({:?})", e.entity)),
            ));
        });

        info!("Spawned visual for unit {:?} at {:?}", e.entity, e.position);
    }
}

/// Sync all unit visual positions when TileConfig changes
pub fn sync_unit_visual_transform(
    cfg: Res<TileConfig>,
    mut unit_query: Query<(&TileCoords, &mut Transform), With<UnitMarker>>,
) {
    for (tile_coords, mut transform) in &mut unit_query {
        let wp = tile_to_world_coords(tile_coords.x, tile_coords.y, &cfg);
        
        // Update parent Transform - child will follow automatically
        transform.translation.x = wp.x;
        transform.translation.y = wp.y;
        // Keep parent z at 0.0, child has z::UNIT
    }
}

/// Sync unit visual positions when only Unit component changes (more efficient)
pub fn sync_unit_visual_on_unit_changed(
    cfg: Res<TileConfig>,
    mut changed_units: Query<(&TileCoords, &mut Transform), (With<UnitMarker>, Changed<TileCoords>)>,
) {
    for (tile_coords, mut transform) in &mut changed_units {
        let wp = tile_to_world_coords(tile_coords.x, tile_coords.y, &cfg);
        
        // Update parent Transform - child will follow automatically
        transform.translation.x = wp.x;
        transform.translation.y = wp.y;
        // Keep parent z at 0.0, child has z::UNIT
    }
}

/// Run condition: check if any TileCoords component changed
pub fn run_if_changed_unit() -> impl FnMut(Query<&TileCoords, (With<UnitMarker>, Changed<TileCoords>)>) -> bool {
    |changed_units: Query<&TileCoords, (With<UnitMarker>, Changed<TileCoords>)>| !changed_units.is_empty()
}


// Note: With Parent/Child hierarchy, no need for manual cleanup!
// When parent (logic unit) is despawned with despawn_recursive(), 
// child (visual sprite) is automatically despawned too.

