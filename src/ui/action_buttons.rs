//! Action buttons UI for unit commands
//!
//! Shows Move and Attack buttons when a unit is selected

use bevy::prelude::*;
use crate::{
    features::tiles::{
        selection::SelectionCtx,
        core::Team,
        units::{
            bundles::UnitMarker,
            components::stats::CurrentStats,
        },
    },
    states::in_game::UnitCommandState,
};

/// Marker component for the action buttons panel
#[derive(Component)]
pub struct ActionButtonsPanel;

/// Marker component for the Move button
#[derive(Component)]
pub struct MoveButton;

/// Marker component for the Attack button
#[derive(Component)]
pub struct AttackButton;

/// Setup action buttons UI (positioned at bottom left)
pub fn setup_action_buttons_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            bottom: Val::Px(20.0),
            width: Val::Px(200.0),
            height: Val::Px(60.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            display: Display::None,
            ..default()
        },
        BackgroundColor(Color::NONE),
        ActionButtonsPanel,
    )).with_children(|parent| {
        // Move Button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(80.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.6, 0.2)), // Green
            MoveButton,
        )).with_children(|button| {
            button.spawn((
                Text::new("Move"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });

        // Attack Button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(80.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.8, 0.2, 0.2)), // Red
            AttackButton,
        )).with_children(|button| {
            button.spawn((
                Text::new("Attack"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    });
}

/// Update action buttons visibility and state based on selection
pub fn update_action_buttons(
    selection: Res<SelectionCtx>,
    unit_query: Query<(&Team, &CurrentStats), With<UnitMarker>>,
    mut panel_query: Query<&mut Node, With<ActionButtonsPanel>>,
    mut move_button_query: Query<(&mut BackgroundColor, &mut Interaction), (With<MoveButton>, Without<AttackButton>)>,
    mut attack_button_query: Query<(&mut BackgroundColor, &mut Interaction), (With<AttackButton>, Without<MoveButton>)>,
    command_state: Res<State<UnitCommandState>>,
) {
    let Ok(mut panel_node) = panel_query.single_mut() else { return; };
    
    // Show buttons only when a player unit is selected
    if let Some(selected_entity) = selection.selected_unit {
        if let Ok((team, current_stats)) = unit_query.get(selected_entity) {
            match team {
                Team::Player => {
                    panel_node.display = Display::Flex;
                    
                    // Update Move button state
                    if let Ok((mut move_color, mut move_interaction)) = move_button_query.single_mut() {
                        let can_move = current_stats.move_range > 0;
                        let is_move_active = command_state.get() == &UnitCommandState::Move;
                        
                        if !can_move || is_move_active {
                            *move_color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4)); // Gray (disabled or active)
                            *move_interaction = Interaction::None; // Reset interaction
                        } else {
                            *move_color = BackgroundColor(Color::srgb(0.2, 0.6, 0.2)); // Green
                        }
                    }
                    
                    // Update Attack button state  
                    if let Ok((mut attack_color, mut attack_interaction)) = attack_button_query.single_mut() {
                        let can_attack = current_stats.actions_per_turn > 0;
                        let is_attack_active = command_state.get() == &UnitCommandState::Attack;
                        
                        if !can_attack || is_attack_active {
                            *attack_color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4)); // Gray (disabled or active)
                            *attack_interaction = Interaction::None; // Reset interaction
                        } else {
                            *attack_color = BackgroundColor(Color::srgb(0.8, 0.2, 0.2)); // Red
                        }
                    }
                }
                _ => {
                    panel_node.display = Display::None;
                }
            }
        } else {
            panel_node.display = Display::None;
        }
    } else {
        panel_node.display = Display::None;
    }
}

/// Handle Move button clicks
pub fn handle_move_button_click(
    mut interaction_query: Query<(&Interaction, &BackgroundColor), (Changed<Interaction>, With<MoveButton>)>,
    selection: Res<SelectionCtx>,
    unit_query: Query<&CurrentStats, With<UnitMarker>>,
    mut next_state: ResMut<NextState<UnitCommandState>>,
) {
    for (interaction, background_color) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // Check if button is enabled (not gray)
            let is_enabled = background_color.0 != Color::srgb(0.4, 0.4, 0.4);
            
            if is_enabled {
                if let Some(selected_entity) = selection.selected_unit {
                    if let Ok(current_stats) = unit_query.get(selected_entity) {
                        if current_stats.move_range > 0 {
                            info!("Move button clicked - switching to Move state");
                            next_state.set(UnitCommandState::Move);
                        }
                    }
                }
            }
        }
    }
}

/// Handle Attack button clicks
pub fn handle_attack_button_click(
    mut interaction_query: Query<(&Interaction, &BackgroundColor), (Changed<Interaction>, With<AttackButton>)>,
    selection: Res<SelectionCtx>,
    unit_query: Query<&CurrentStats, With<UnitMarker>>,
    mut next_state: ResMut<NextState<UnitCommandState>>,
) {
    for (interaction, background_color) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // Check if button is enabled (not gray)
            let is_enabled = background_color.0 != Color::srgb(0.4, 0.4, 0.4);
            
            if is_enabled {
                if let Some(selected_entity) = selection.selected_unit {
                    if let Ok(current_stats) = unit_query.get(selected_entity) {
                        if current_stats.actions_per_turn > 0 {
                            info!("Attack button clicked - switching to Attack state");
                            next_state.set(UnitCommandState::Attack);
                        }
                    }
                }
            }
        }
    }
}

/// Update button colors on hover (only for enabled buttons)
pub fn update_button_colors(
    selection: Res<SelectionCtx>,
    unit_query: Query<&CurrentStats, With<UnitMarker>>,
    command_state: Res<State<UnitCommandState>>,
    mut move_button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<MoveButton>, Without<AttackButton>)>,
    mut attack_button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<AttackButton>, Without<MoveButton>)>,
) {
    // Get current unit stats and command state to determine if buttons should be enabled
    let (can_move, can_attack) = if let Some(selected_entity) = selection.selected_unit {
        if let Ok(current_stats) = unit_query.get(selected_entity) {
            (current_stats.move_range > 0, current_stats.actions_per_turn > 0)
        } else {
            (false, false)
        }
    } else {
        (false, false)
    };
    
    let is_move_active = command_state.get() == &UnitCommandState::Move;
    let is_attack_active = command_state.get() == &UnitCommandState::Attack;
    
    // Update Move button colors
    for (interaction, mut color) in move_button_query.iter_mut() {
        if can_move && !is_move_active {
            *color = match *interaction {
                Interaction::Pressed => BackgroundColor(Color::srgb(0.1, 0.4, 0.1)), // Dark green
                Interaction::Hovered => BackgroundColor(Color::srgb(0.3, 0.7, 0.3)), // Light green
                Interaction::None => BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),    // Normal green
            };
        } else {
            // Keep disabled/active color regardless of interaction
            *color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4)); // Gray
        }
    }
    
    // Update Attack button colors
    for (interaction, mut color) in attack_button_query.iter_mut() {
        if can_attack && !is_attack_active {
            *color = match *interaction {
                Interaction::Pressed => BackgroundColor(Color::srgb(0.6, 0.1, 0.1)), // Dark red
                Interaction::Hovered => BackgroundColor(Color::srgb(0.9, 0.3, 0.3)), // Light red
                Interaction::None => BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),    // Normal red
            };
        } else {
            // Keep disabled/active color regardless of interaction
            *color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4)); // Gray
        }
    }
}

/// Cleanup action buttons UI when exiting game state
pub fn cleanup_action_buttons_ui(
    mut commands: Commands,
    query: Query<Entity, With<ActionButtonsPanel>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}