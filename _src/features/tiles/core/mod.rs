//! Core tile system module
//!
//! This module contains core tile functionality organized into:
//! - components: Tile, TileKind, TileCoords and related core components
//! - resources: TileMap, TileConfig and core tile resources
//! - systems: Spawn and coordinate conversion systems
//! - events: Grid update events for spatial index
//! - index: High-performance spatial index (GridIndex)
//! - sets: System scheduling and ordering

use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;
pub mod events;
pub mod sets;

pub use components::*;
pub use resources::*;
pub use systems::*;
pub use events::*;
pub use sets::*;

/// Core tiles plugin that provides spatial indexing and basic tile functionality
pub struct CoreTilesPlugin;

impl Plugin for CoreTilesPlugin {
    fn build(&self, app: &mut App) {
        // Configure system sets
        sets::configure_system_sets(app);
        
        // Register events
        app
            .add_event::<UnitSpawned>()
            .add_event::<UnitDespawned>()
            .add_event::<TileMoved>()
            .add_event::<TileBlockedChanged>()
            .add_event::<TileCostChanged>()
            .add_event::<MapRebuilt>();
        
        // Initialize resources  
        app
            .init_resource::<TileConfig>()
            .insert_resource(TileMap::new(11)) // Initialize with 11x11 grid for Level 1
            .insert_resource(GridIndex::new(11, 11)); // Match TileMap size
        
        // Add index synchronization system
        app.add_systems(PostUpdate, 
            apply_index_updates.in_set(TilesSet::SyncIndex)
        );
    }
}