//! Plugin for UI systems

use bevy::prelude::*;
use crate::states::AppState;
use super::{
    state_display::{setup_state_display_ui, update_state_display},
    hover_info::{setup_hover_info_ui, update_hover_info, cleanup_hover_info_ui},
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Setup UI when entering InGame state
            .add_systems(OnEnter(AppState::InGame), (
                setup_state_display_ui,
                setup_hover_info_ui,
            ))
            
            // Cleanup UI when exiting InGame state
            .add_systems(OnExit(AppState::InGame), cleanup_hover_info_ui)
            
            // Update UI every frame while in InGame
            .add_systems(Update, (
                update_state_display,
                update_hover_info,
            ).run_if(in_state(AppState::InGame)));
    }
}