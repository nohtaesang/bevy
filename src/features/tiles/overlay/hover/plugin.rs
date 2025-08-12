//! Hover plugin

use bevy::prelude::*;
use crate::{
    core::AppState,
    resources::TileConfig,
};
use super::{create_hover_overlay_sprite, tile_hover_system};

/// Hover plugin that handles hover overlay functionality
pub struct HoverPlugin;

impl Plugin for HoverPlugin {
    fn build(&self, app: &mut App) {
        app
            // Setup hover overlay when entering InGame state
            .add_systems(OnEnter(AppState::InGame), setup_hover_overlay)
            
            // Hover system that runs every frame while in InGame state
            .add_systems(Update, tile_hover_system.run_if(in_state(AppState::InGame)));
    }
}

fn setup_hover_overlay(mut commands: Commands, tile_config: Res<TileConfig>) {
    create_hover_overlay_sprite(&mut commands, tile_config.tile_size);
}