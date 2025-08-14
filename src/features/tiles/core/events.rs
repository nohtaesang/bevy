//! Core tile events
//!
//! Events that drive GridIndex updates to maintain spatial awareness

use bevy::prelude::*;

/// Team component for distinguishing player and enemy units
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Team {
    Player,
    Enemy,
}

/// Event fired when a unit is spawned
#[derive(Event, Debug, Clone)]
pub struct UnitSpawned {
    pub entity: Entity,
    pub position: IVec2,
    pub team: Team,
}

/// Event fired when a unit is despawned
#[derive(Event, Debug, Clone)]
pub struct UnitDespawned {
    pub entity: Entity,
    pub position: IVec2,
}

/// Event fired when a unit moves from one tile to another
#[derive(Event, Debug, Clone)]
pub struct TileMoved {
    pub entity: Entity,
    pub from: IVec2,
    pub to: IVec2,
    pub team: Team,
}

/// Event fired when tile blocking status changes
#[derive(Event, Debug, Clone)]
pub struct TileBlockedChanged {
    pub position: IVec2,
    pub blocked: bool,
}

/// Event fired when tile movement cost changes
#[derive(Event, Debug, Clone)]
pub struct TileCostChanged {
    pub position: IVec2,
    pub cost: u8,
}

/// Event fired when the map is rebuilt/resized
/// Visual systems should listen to this event to recreate tile sprites
#[derive(Event, Debug, Clone)]
pub struct MapRebuilt {
    pub width: i32,
    pub height: i32,
}