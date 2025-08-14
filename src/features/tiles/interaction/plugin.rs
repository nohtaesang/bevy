//! Interaction plugin
//!
//! Handles all user input and updates interaction state resources

use bevy::prelude::*;
use crate::{
    states::{AppState, in_game::{TurnState, UnitCommandState}},
    features::tiles::{
        core::TilesSet,
        selection::SelectionCtx,
    },
};
use super::{
    resources::{HoverTile, MovementValidation, AttackValidation},
    events::{TileClicked, ClickTargetEvent},
    systems::{update_hover_tile, emit_tile_clicked, classify_click_target, handle_right_click_state_transition, update_ally_movement_range, update_ally_attack_range},
    hotkeys::unit_command_hotkeys,
};

/// Clear movement validation when exiting Move state
fn clear_movement_validation(mut movement_validation: ResMut<MovementValidation>) {
    movement_validation.clear();
}

/// Clear attack validation when exiting Attack state
fn clear_attack_validation(mut attack_validation: ResMut<AttackValidation>) {
    attack_validation.clear();
}

/// Plugin for tile interaction systems
pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<HoverTile>()
            .init_resource::<MovementValidation>()
            .init_resource::<AttackValidation>()
            
            // Add interaction events
            .add_event::<TileClicked>()
            .add_event::<ClickTargetEvent>()
            
            // Hover tracking system - updates HoverTile resource from cursor input
            .add_systems(Update, (
                update_hover_tile,
                unit_command_hotkeys,
                handle_right_click_state_transition,
            ).run_if(in_state(AppState::InGame))
                .run_if(in_state(TurnState::PlayerTurn))
            )
            
            // Click handling systems - run in PreUpdate/TilesSet::Input
            .add_systems(PreUpdate, (
                emit_tile_clicked,
                classify_click_target,
            ).chain()
                .in_set(TilesSet::Input)
                .run_if(in_state(AppState::InGame))
            )
            
            // Movement range calculation
            .add_systems(Update, 
                update_ally_movement_range
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(UnitCommandState::Move))
                    .run_if(resource_changed::<SelectionCtx>)
            )
            
            // Movement range calculation when entering Move state
            .add_systems(OnEnter(UnitCommandState::Move),
                update_ally_movement_range
                    .run_if(in_state(AppState::InGame))
            )
            
            // Attack range calculation
            .add_systems(Update,
                update_ally_attack_range
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(UnitCommandState::Attack))
                    .run_if(resource_changed::<SelectionCtx>)
            )
            
            // Attack range calculation when entering Attack state
            .add_systems(OnEnter(UnitCommandState::Attack),
                update_ally_attack_range
                    .run_if(in_state(AppState::InGame))
            )
            
            // Clear validations when exiting states
            .add_systems(OnExit(UnitCommandState::Move), clear_movement_validation)
            .add_systems(OnExit(UnitCommandState::Attack), clear_attack_validation);
    }
}