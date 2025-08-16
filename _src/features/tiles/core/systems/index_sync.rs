//! Grid index synchronization system
//!
//! Handles event-driven updates to the GridIndex spatial cache.
//! Runs in PreUpdate to ensure index is synchronized before consumers read it.

use bevy::prelude::*;
use super::super::{
    events::*,
    resources::GridIndex,
};

/// System that applies index updates from events
/// Runs in PreUpdate to ensure index is synchronized before consumers read it
pub fn apply_index_updates(
    mut grid_index: ResMut<GridIndex>,
    mut unit_spawned: EventReader<UnitSpawned>,
    mut unit_despawned: EventReader<UnitDespawned>,
    mut tile_moved: EventReader<TileMoved>,
    mut tile_blocked_changed: EventReader<TileBlockedChanged>,
    mut tile_cost_changed: EventReader<TileCostChanged>,
) {
    // Process unit spawns
    for event in unit_spawned.read() {
        grid_index.set_unit_at(event.position, Some(event.entity), Some(event.team));
    }
    
    // Process unit despawns
    for event in unit_despawned.read() {
        grid_index.set_unit_at(event.position, None, None);
    }
    
    // Process unit moves
    for event in tile_moved.read() {
        // Clear old position
        grid_index.set_unit_at(event.from, None, None);
        // Set new position
        grid_index.set_unit_at(event.to, Some(event.entity), Some(event.team));
    }
    
    // Process blocked changes
    for event in tile_blocked_changed.read() {
        grid_index.set_blocked(event.position, event.blocked);
    }
    
    // Process cost changes
    for event in tile_cost_changed.read() {
        grid_index.set_cost(event.position, event.cost);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_occupancy_and_version() {
        let mut index = GridIndex::new(10, 10);
        let initial_version = index.version();
        
        let pos = IVec2::new(5, 5);
        let entity = Entity::from_raw(42);
        
        // Initially empty
        assert!(index.is_empty(pos));
        assert_eq!(index.unit_at(pos), None);
        assert_eq!(index.team_at(pos), None);
        
        // Set unit
        index.set_unit_at(pos, Some(entity), Some(Team::Player));
        
        // Verify occupancy
        assert!(!index.is_empty(pos));
        assert_eq!(index.unit_at(pos), Some(entity));
        assert_eq!(index.team_at(pos), Some(Team::Player));
        
        // Verify version increment
        assert_eq!(index.version(), initial_version + 1);
        
        // Clear unit
        index.set_unit_at(pos, None, None);
        
        // Verify empty again
        assert!(index.is_empty(pos));
        assert_eq!(index.unit_at(pos), None);
        assert_eq!(index.team_at(pos), None);
        
        // Verify another version increment
        assert_eq!(index.version(), initial_version + 2);
    }
    
    #[test]
    fn test_bounds_checking() {
        let index = GridIndex::new(5, 5);
        
        // In bounds
        assert!(index.is_in_bounds(IVec2::new(0, 0)));
        assert!(index.is_in_bounds(IVec2::new(4, 4)));
        
        // Out of bounds
        assert!(!index.is_in_bounds(IVec2::new(-1, 0)));
        assert!(!index.is_in_bounds(IVec2::new(5, 0)));
        assert!(!index.is_in_bounds(IVec2::new(0, -1)));
        assert!(!index.is_in_bounds(IVec2::new(0, 5)));
        
        // Out of bounds positions should be blocked with max cost
        assert!(index.is_blocked(IVec2::new(-1, -1)));
        assert_eq!(index.cost_at(IVec2::new(-1, -1)), 255);
    }
}