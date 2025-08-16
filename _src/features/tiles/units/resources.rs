//! Unit resources
//!
//! This module contains unit-related resources and configurations

use bevy::prelude::*;

/// Configuration for unit spawning and behavior
#[derive(Resource)]
pub struct UnitConfig {
    pub ally_spawn_area: (IVec2, IVec2), // (min, max) spawn coordinates for allies
    pub enemy_spawn_area: (IVec2, IVec2), // (min, max) spawn coordinates for enemies
    pub max_enemies: usize,
}

impl Default for UnitConfig {
    fn default() -> Self {
        Self {
            ally_spawn_area: (IVec2::new(0, 0), IVec2::new(9, 9)),
            enemy_spawn_area: (IVec2::new(10, 0), IVec2::new(50, 50)),
            max_enemies: 500,
        }
    }
}