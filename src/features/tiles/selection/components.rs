//! Selection-related components
//!
//! This module contains component definitions for tile selection functionality

use bevy::prelude::*;

/// Component for entities that can be selected
#[derive(Component)]
pub struct Selectable;

/// Component marking an entity as currently selected
#[derive(Component)]
pub struct Selection;

/// Component for tracking cursor hit on tiles
#[derive(Component)]
pub struct CursorHit;