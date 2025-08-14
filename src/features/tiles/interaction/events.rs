//! Interaction events
//!
//! Events for user input that other systems can respond to

use bevy::prelude::*;

/// Raw tile click event - emitted when user clicks on the game world
#[derive(Event, Debug, Clone, Copy)]
pub struct TileClicked {
    /// Grid coordinates (None if outside grid)
    pub tile_pos: Option<IVec2>,
    /// World coordinates (for tooltips, effects, etc.)
    pub world_pos: Vec2,
    /// Mouse button that was clicked
    pub button: MouseButton,
}

/// Click target classification (priority: overlay > unit/enemy > empty > outside)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickTarget {
    /// Clicked outside the grid
    OutsideGrid,
    /// Clicked on a movement overlay tile
    MovementOverlay(IVec2),
    /// Clicked on an attack overlay tile
    AttackOverlay(IVec2),
    /// Clicked on the currently selected unit
    SelfUnit(IVec2, Entity),
    /// Clicked on a friendly unit (not selected)
    FriendlyUnit(IVec2, Entity),
    /// Clicked on an enemy unit
    Enemy(IVec2, Entity),
    /// Clicked on an empty tile
    EmptyTile(IVec2),
}

/// Event with classified click target - consumed by selection handlers
#[derive(Event, Debug, Clone, Copy)]
pub struct ClickTargetEvent(pub ClickTarget);