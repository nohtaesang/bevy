//! Unit information UI display
//!
//! Shows detailed information about the currently selected unit

use bevy::prelude::*;
use crate::features::tiles::{
    selection::SelectionCtx,
    units::components::{Unit, AttackDirection, AttackType, AttackRange},
    core::Team,
};

/// Marker component for the unit info UI panel
#[derive(Component)]
pub struct UnitInfoPanel;

/// Marker component for unit info text elements
#[derive(Component)]
pub struct UnitInfoText;

/// Setup the unit info UI panel
pub fn setup_unit_info_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            width: Val::Px(300.0),
            height: Val::Auto,
            padding: UiRect::all(Val::Px(15.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
        BorderColor(Color::srgb(0.3, 0.3, 0.3)),
        BorderRadius::all(Val::Px(8.0)),
        Visibility::Hidden, // Initially hidden
        UnitInfoPanel,
        Name::new("UnitInfoPanel"),
    ))
    .with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("Unit Information"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
        ));
        
        // Unit info text area
        parent.spawn((
            Text::new("No unit selected"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            UnitInfoText,
        ));
    });
}

/// Update unit info display when selection changes
pub fn update_unit_info(
    selection: Res<SelectionCtx>,
    unit_query: Query<(&Unit, Option<&Team>)>,
    mut panel_query: Query<&mut Visibility, With<UnitInfoPanel>>,
    mut text_query: Query<&mut Text, With<UnitInfoText>>,
) {
    let Ok(mut panel_visibility) = panel_query.single_mut() else { return; };
    let Ok(mut info_text) = text_query.single_mut() else { return; };
    
    // Check if a unit is selected
    if let Some(selected_entity) = selection.selected_unit {
        if let Ok((unit, team)) = unit_query.get(selected_entity) {
                // Show panel
                *panel_visibility = Visibility::Visible;
                
                // Format unit information
                let team_name = match team {
                    Some(Team::Player) => "Player",
                    Some(Team::Enemy) => "Enemy",
                    None => "Unknown",
                };
                
                let attack_dir_str = match unit.attack_direction {
                    AttackDirection::Cardinal => "Cardinal (↑↓←→)",
                    AttackDirection::EightWay => "Eight Way (↑↓←→↖↗↙↘)",
                };
                
                let attack_type_str = match unit.attack_type {
                    AttackType::Direct => "Direct Combat",
                    AttackType::Indirect => "Indirect Combat",
                };
                
                **info_text = format!(
                    "Position: ({}, {})\n\
                    Team: {}\n\
                    \n\
                    COMBAT INFO:\n\
                    Attack Direction: {}\n\
                    Attack Type: {}\n\
                    Min Range: {}\n\
                    Max Range: {}",
                    unit.tile_pos.x, unit.tile_pos.y,
                    team_name,
                    attack_dir_str,
                    attack_type_str,
                    unit.attack_range.min,
                    unit.attack_range.max
                );
        } else {
            // Selected entity is not a unit
            *panel_visibility = Visibility::Hidden;
        }
    } else {
        // No unit selection
        *panel_visibility = Visibility::Hidden;
    }
}

/// Cleanup unit info UI when exiting game state
pub fn cleanup_unit_info_ui(
    mut commands: Commands,
    query: Query<Entity, With<UnitInfoPanel>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}