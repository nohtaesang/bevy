//! Visual overlay systems
//!
//! This module contains visual overlay functionality organized into:
//! - components: Overlay components and marker types
//! - resources: OverlayState and validation resources  
//! - systems: Hover, selection, movement, and attack overlays
//! - plugin: Visual systems plugin

pub mod components;
pub mod resources;
pub mod systems;
pub mod plugin;
pub mod z_layers;

// Re-export main components for easy access
pub use components::{
    HoverOverlay, SelectedOverlay, MovementOverlay, AttackOverlay,
    Overlay, HoverMarker,
};

// Re-export resources
pub use resources::{
    MovementOverlayState, AttackOverlayState,
};

// Re-export systems
pub use systems::{
    // Hover systems
    render_hover_overlay,
    spawn_hover_overlay,
    ensure_hover_overlay,
    // Selection systems
    render_selection_overlay,
    spawn_selection_overlay,
    ensure_selection_overlay,
    // Movement systems
    cleanup_movement_overlays,
    movement_overlay_system,
    // Attack systems
    cleanup_attack_overlays,
    attack_overlay_system,
    // Tile rendering systems
    rebuild_visual_tiles_on_map_event,
    // Unit rendering systems
    spawn_unit_visual_on_spawned,
    sync_unit_visual_transform,
    sync_unit_visual_on_unit_changed,
    run_if_changed_unit,
};

// Re-export plugin
pub use plugin::VisualPlugin;

// Re-export Z-layer constants
pub use z_layers::z;