//! Visual plugin
//!
//! Handles all tile rendering and visual overlays

use bevy::prelude::*;
use crate::states::{AppState, in_game::UnitCommandState};
use super::systems::{
    spawn_hover_overlay, 
    ensure_hover_overlay,
    render_hover_overlay,
    spawn_selection_overlay,
    ensure_selection_overlay,
    render_selection_overlay,
    rebuild_visual_tiles_on_map_event,
    spawn_unit_visual_on_spawned,
    sync_unit_visual_transform,
    sync_unit_visual_on_unit_changed,
    run_if_changed_unit,
    movement_overlay_system,
    cleanup_movement_overlays,
    attack_overlay_system,
    cleanup_attack_overlays,
};

/// Plugin for tile visual systems
pub struct VisualPlugin;

impl Plugin for VisualPlugin {
    fn build(&self, app: &mut App) {
        app
            // Visual tile rendering system - responds to MapRebuilt events
            .add_systems(Update, 
                rebuild_visual_tiles_on_map_event.run_if(in_state(AppState::InGame))
            )
            
            // Hover overlay systems - reads HoverTile resource (read-only)
            .add_systems(OnEnter(AppState::InGame), spawn_hover_overlay)
            .add_systems(Update, (
                // Ensure overlay exists if we're trying to hover (only checks when HoverTile changes)
                ensure_hover_overlay
                    .run_if(resource_changed::<crate::features::tiles::interaction::prelude::HoverTile>),
                // Render the hover overlay
                render_hover_overlay
                    .run_if(resource_changed::<crate::features::tiles::interaction::prelude::HoverTile>)
                    // Note: We calculate world position from tile coords each frame,
                    // so TileConfig changes are automatically handled
            ).chain().run_if(in_state(AppState::InGame)))
            
            // Selection overlay systems - reads SelectionCtx resource (read-only)
            .add_systems(OnEnter(AppState::InGame), spawn_selection_overlay)
            .add_systems(Update, (
                // Ensure overlay exists if we have something selected
                ensure_selection_overlay
                    .run_if(resource_changed::<crate::features::tiles::selection::SelectionCtx>),
                // Render the selection overlay
                render_selection_overlay
                    .run_if(resource_changed::<crate::features::tiles::selection::SelectionCtx>)
                    // Note: We calculate world position from tile coords each frame,
                    // so TileConfig changes are automatically handled
            ).chain().run_if(in_state(AppState::InGame)))
            
            // Unit visual rendering systems - event-driven and efficient
            .add_systems(Update, (
                spawn_unit_visual_on_spawned,      // Event-based spawning
                sync_unit_visual_on_unit_changed   // Only when Unit components change
                    .run_if(run_if_changed_unit()),
                sync_unit_visual_transform         // When TileConfig changes, sync all
                    .run_if(resource_changed::<crate::features::tiles::core::TileConfig>),
            ).run_if(in_state(AppState::InGame)))
            
            // Movement overlay systems - show green tiles for valid moves
            .add_systems(Update,
                movement_overlay_system
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(UnitCommandState::Move))
                    .run_if(resource_changed::<crate::features::tiles::interaction::MovementValidation>)
            )
            .add_systems(OnExit(UnitCommandState::Move), cleanup_movement_overlays)
            
            // Attack overlay systems - show red tiles for valid attacks  
            .add_systems(Update,
                attack_overlay_system
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(UnitCommandState::Attack))
                    .run_if(resource_changed::<crate::features::tiles::interaction::AttackValidation>)
            )
            .add_systems(OnExit(UnitCommandState::Attack), cleanup_attack_overlays);
            
        // Note: This plugin ONLY reads resources, never modifies game logic
        // Game logic is handled by other plugins (interaction, selection, etc.)
    }
}