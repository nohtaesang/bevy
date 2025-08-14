//! Level 1 scenario - 11x11 grid with player in center
//!
//! This scenario creates a simple 11x11 grid map with a single player unit
//! spawned in the center position.

use bevy::prelude::*;
use crate::{
    states::AppState,
    features::tiles::{
        core::{TileConfig, TileMap, tile_to_world_coords, UnitSpawned, Team, GridIndex, MapRebuilt},
        units::components::{Unit, AttackDirection, AttackType, AttackRange},
        visual::z,
    },
};

/// Plugin for Level 1 scenario
pub struct Level1Plugin;

impl Plugin for Level1Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), (
            setup_level_1_grid,
            spawn_level_1_player,
        ).chain());
    }
}

fn setup_level_1_grid(
    mut tile_config: ResMut<TileConfig>,
    mut tile_map: ResMut<TileMap>,
    mut commands: Commands,
    existing_index: Option<ResMut<GridIndex>>,
    mut map_rebuilt: EventWriter<MapRebuilt>,
) {
    let grid_size = 11;
    
    tile_config.grid_size = grid_size;
    *tile_map = TileMap::new(grid_size);

    if let Some(mut idx) = existing_index {
        *idx = GridIndex::new(grid_size, grid_size);
    } else {
        commands.insert_resource(GridIndex::new(grid_size, grid_size));
    }
    
    // Emit MapRebuilt event to trigger visual tile spawning
    map_rebuilt.send(MapRebuilt {
        width: grid_size,
        height: grid_size,
    });
    
    info!("Level 1: Map rebuilt with size {}x{}", grid_size, grid_size);
}


/// Spawn the player unit in the center of the 11x11 grid
fn spawn_level_1_player(
    mut commands: Commands,
    tile_config: Res<TileConfig>,
    mut tile_map: ResMut<TileMap>,
    mut unit_spawned: EventWriter<UnitSpawned>,
) {
    // Center position in 11x11 grid (0-indexed, so center is at 5,5)
    let center_pos = IVec2::new(5, 5);
    
    // Spawn player unit with balanced stats and team component (logic only)
    let player_entity = commands.spawn((
        Unit::new(
            center_pos,
            AttackDirection::EightWay, // Can attack in all 8 directions
            AttackType::Direct,        // Direct combat
            AttackRange::new(1, 2),    // Attack range 1-2 tiles
        ),
        Team::Player,               // Team component for entity
        Name::new("Player"),
    )).id();
    
    // Register unit in tile map with safety check
    if !tile_map.place_unit(center_pos, player_entity) {
        warn!("Failed to place player unit at {:?} - position may be occupied", center_pos);
        commands.entity(player_entity).despawn();
        return;
    }
    
    // Emit spawn event for GridIndex
    unit_spawned.send(UnitSpawned {
        entity: player_entity,
        position: center_pos,
        team: Team::Player,
    });
    
    info!("Level 1: Spawned player at center position {:?}", center_pos);
}