//! Plugin for UI systems

use bevy::prelude::*;
use crate::states::{AppState, in_game::UnitCommandState};
use super::{
    state_display::{setup_state_display_ui, update_state_display},
    hover_info::{setup_hover_info_ui, update_hover_info, cleanup_hover_info_ui},
    unit_info::{setup_unit_info_ui, update_unit_info, cleanup_unit_info_ui},
    action_buttons::{
        setup_action_buttons_ui, update_action_buttons, handle_move_button_click, 
        handle_attack_button_click, update_button_colors, cleanup_action_buttons_ui
    },
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Setup UI when entering InGame state
            .add_systems(OnEnter(AppState::InGame), (
                setup_state_display_ui,
                setup_hover_info_ui,
                setup_unit_info_ui,
                setup_action_buttons_ui,
            ))
            
            // Cleanup UI when exiting InGame state
            .add_systems(OnExit(AppState::InGame), (
                cleanup_hover_info_ui,
                cleanup_unit_info_ui,
                cleanup_action_buttons_ui,
            ))
            
            // Update UI every frame while in InGame
            .add_systems(Update, (
                update_state_display,
                update_hover_info,
                update_unit_info.run_if(resource_changed::<crate::features::tiles::selection::SelectionCtx>),
                update_action_buttons,
                handle_move_button_click,
                handle_attack_button_click,
                update_button_colors,
            ).run_if(in_state(AppState::InGame)));
    }
}