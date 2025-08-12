//! UI system for displaying current game state

use bevy::prelude::*;
use crate::{
    core::{TurnState, SelectionState, ActionState, SelectionCtx},
    features::units::{Unit, Enemy},
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
    selection_state: Res<State<SelectionState>>,
    action_state: Res<State<ActionState>>,
    selection_ctx: Res<SelectionCtx>,
    unit_query: Query<&Unit>,
    enemy_query: Query<&Enemy>,
) {
    for mut text in text_query.iter_mut() {
        let mut display_text = String::new();
        
        // Turn state
        display_text.push_str(&format!("Turn: {:?}\n", turn_state.get()));
        
        // Selection state
        display_text.push_str(&format!("Selection: {:?}\n", selection_state.get()));
        
        // Action state
        display_text.push_str(&format!("Action: {:?}\n", action_state.get()));
        
        // Selected tile
        if let Some(tile_pos) = selection_ctx.tile {
            display_text.push_str(&format!("Tile: ({}, {})\n", tile_pos.x, tile_pos.y));
        } else {
            display_text.push_str("Tile: None\n");
        }
        
        // Selected entity details
        if let Some(entity) = selection_ctx.selected_unit {
            if let Ok(unit) = unit_query.get(entity) {
                display_text.push_str(&format!("\nUnit Selected:\n"));
                display_text.push_str(&format!("  HP: {}/{}\n", unit.health, unit.max_health));
                display_text.push_str(&format!("  ATK: {}\n", unit.attack));
                display_text.push_str(&format!("  Move: {}\n", unit.movement_range));
            }
        } else if let Some(entity) = selection_ctx.selected_enemy {
            if let Ok(enemy) = enemy_query.get(entity) {
                display_text.push_str(&format!("\nEnemy Selected:\n"));
                display_text.push_str(&format!("  HP: {}/{}\n", enemy.health, enemy.max_health));
                display_text.push_str(&format!("  ATK: {}\n", enemy.attack));
            }
        }
        
        text.0 = display_text;
    }
}