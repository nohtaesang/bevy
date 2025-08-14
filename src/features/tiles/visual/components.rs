//! Visual overlay components
//!
//! This module contains overlay components and marker types

use bevy::prelude::*;

/// Component for hover overlay tiles - shows when cursor hovers over a tile
#[derive(Component)]
pub struct HoverOverlay;

/// Component for selected overlay tiles - shows the currently selected tile
#[derive(Component)]
pub struct SelectedOverlay;

/// Component for movement range overlay tiles - shows where a unit can move
#[derive(Component)]
pub struct MovementOverlay {
    pub tile_pos: IVec2,
}

/// Component for attack range overlay tiles - shows where a unit can attack
#[derive(Component)]
pub struct AttackOverlay {
    pub tile_pos: IVec2,
}

/// Marker component for overlay entities
#[derive(Component)]
pub struct Overlay;

/// Marker component for hover-related entities
#[derive(Component)]
pub struct HoverMarker;

// visual/components.rs
use bevy::prelude::*;

#[derive(Component)]
pub struct UnitVisual;               // 실제 스프라이트에 달리는 마커

// Note: UnitVisualRef is no longer needed with Parent/Child hierarchy