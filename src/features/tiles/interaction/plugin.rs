//! Interaction plugin
//!
//! Handles all user input and updates interaction state resources

use bevy::prelude::*;
use crate::{
    states::AppState,
    features::tiles::core::TilesSet,
};
use super::{
    resources::HoverTile,
    events::{TileClicked, ClickTargetEvent},
    systems::{update_hover_tile, emit_tile_clicked, classify_click_target},
};

/// Plugin for tile interaction systems
pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<HoverTile>()
            
            // Add interaction events
            .add_event::<TileClicked>()
            .add_event::<ClickTargetEvent>()
            
            // Hover tracking system - updates HoverTile resource from cursor input
            .add_systems(Update, 
                update_hover_tile.run_if(in_state(AppState::InGame))
            )
            
            // Click handling systems - run in PreUpdate/TilesSet::Input
            .add_systems(PreUpdate, (
                emit_tile_clicked,
                classify_click_target,
            ).chain()
                .in_set(TilesSet::Input)
                .run_if(in_state(AppState::InGame))
            );
    }
}