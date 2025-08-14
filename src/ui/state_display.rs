//! UI system for displaying current game state

use bevy::prelude::*;
use crate::{
    states::in_game::{TurnState, SelectionState, UnitCommandState},
    features::tiles::{SelectionCtx, Team, units::bundles::UnitMarker},
};

#[derive(Component)]
pub struct StateDisplayUI;

#[derive(Component)]
pub struct StateText;

pub fn setup_state_display_ui(mut commands: Commands) {
    // Create UI root node on the left side
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        StateDisplayUI,
    ))
    .with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("Game State"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
        
        // State text (will be updated dynamically)
        parent.spawn((
            Text::new("Loading..."),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
            StateText,
        ));
    });
}

pub fn update_state_display(
    mut text_query: Query<&mut Text, With<StateText>>,
    turn_state: Res<State<TurnState>>,
    selection_state: Option<Res<State<SelectionState>>>,
    action_state: Option<Res<State<UnitCommandState>>>,
    selection_ctx: Option<Res<SelectionCtx>>,
    _unit_query: Query<&Team, With<UnitMarker>>,
) {
    for mut text in text_query.iter_mut() {
        let mut display_text = String::new();
        
        // Turn state
        display_text.push_str(&format!("Turn: {:?}\n", turn_state.get()));
        
        // Selection state
        if let Some(selection_state) = &selection_state {
            display_text.push_str(&format!("Selection: {:?}\n", selection_state.get()));
        } else {
            display_text.push_str("Selection: N/A\n");
        }
        
        // Action state
        if let Some(action_state) = &action_state {
            display_text.push_str(&format!("Action: {:?}\n", action_state.get()));
        } else {
            display_text.push_str("Action: N/A\n");
        }
        
        // Selected tile
        if let Some(selection_ctx) = &selection_ctx {
            if let Some(tile_pos) = selection_ctx.tile {
                display_text.push_str(&format!("Tile: ({}, {})\n", tile_pos.x, tile_pos.y));
            } else {
                display_text.push_str("Tile: None\n");
            }
            
            // Selected entity details
            if let Some(entity) = selection_ctx.selected_unit {
                // TODO: Get unit info from proper components and display
                display_text.push_str("\nUnit Selected (info not available yet)\n");
            } else if let Some(entity) = selection_ctx.selected_enemy {
                // TODO: Get enemy info from proper components and display
                display_text.push_str("\nEnemy Selected (info not available yet)\n");
            }
        } else {
            display_text.push_str("Tile: N/A\n");
        }
        
        display_text.push_str("\nPress SPACE to switch turns");
        
        text.0 = display_text;
    }
}