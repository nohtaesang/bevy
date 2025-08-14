//! Z-layer constants for visual ordering
//!
//! Defines the Z-depth layers for all visual elements in the tile system

/// Z-layer constants for consistent depth ordering
pub mod z {
    /// Background tiles - the lowest layer
    pub const TILE: f32 = 0.0;
    
    /// Grid lines and debug visuals
    pub const GRID: f32 = 0.1;
    
    /// Units, enemies, and game objects
    pub const UNIT: f32 = 1.0;
    
    /// Visual effects (damage, animations, etc.)
    pub const EFFECT: f32 = 2.0;
    
    /// Selection overlay (shows selected tile)
    pub const SELECTION: f32 = 3.0;
    
    /// Movement and attack range overlays
    pub const OVERLAY: f32 = 4.0;
    
    /// Hover overlay (follows cursor)
    pub const HOVER: f32 = 5.0;
    
    /// Cursor and selection indicators
    pub const CURSOR: f32 = 6.0;
    
    /// World-space UI elements (health bars, tooltips)
    pub const UI_WORLD: f32 = 10.0;
    
    // Note: Bevy UI (Node-based) uses a separate rendering pipeline
    // and doesn't need Z-layer constants here
}