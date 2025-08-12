//! Selected plugin

use bevy::prelude::*;
use crate::{
    states::AppState,
    features::tiles::TileConfig,
};
use super::{create_selected_overlay_sprite, selected_overlay_system};

/// Selected plugin that handles selected overlay functionality
pub struct SelectedOverlayPlugin;

impl Plugin for SelectedOverlayPlugin {
    fn build(&self, app: &mut App) {
        app
            // Setup selected overlay when entering InGame state
            .add_systems(OnEnter(AppState::InGame), setup_selected_overlay)
            
            // Selected system that runs every frame while in InGame state
            .add_systems(Update, selected_overlay_system.run_if(in_state(AppState::InGame)));
    }
}

fn setup_selected_overlay(mut commands: Commands, tile_config: Res<TileConfig>) {
    create_selected_overlay_sprite(&mut commands, tile_config.tile_size);
}