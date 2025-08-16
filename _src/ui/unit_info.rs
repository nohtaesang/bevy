//! Unit information UI display
//!
//! Shows detailed information about the currently selected unit

use bevy::prelude::*;
use crate::features::tiles::{
    selection::SelectionCtx,
    units::{
        bundles::UnitMarker,
        components::{
            attack_profile::{AttackDirection, AttackType},
            stats::{BaseStats, CurrentStats},
        },
    },
    core::Team,
};

/// Marker component for the unit info UI panel
#[derive(Component)]
pub struct UnitInfoPanel;

/// Marker component for the unit info text
#[derive(Component)]
pub struct UnitInfoText;

/// Setup unit info UI panel (positioned on the right side)
pub fn setup_unit_info_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            width: Val::Px(250.0),
            height: Val::Auto,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        Visibility::Hidden,
        UnitInfoPanel,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("No Unit Selected"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::WHITE),
            UnitInfoText,
        ));
    });
}

/// Update unit info display when selection changes
pub fn update_unit_info(
    selection: Res<SelectionCtx>,
    _unit_query: Query<&Team, With<UnitMarker>>,
    mut panel_query: Query<&mut Visibility, With<UnitInfoPanel>>,
    mut text_query: Query<&mut Text, With<UnitInfoText>>,
) {
    let Ok(mut panel_visibility) = panel_query.single_mut() else { return; };
    let Ok(mut info_text) = text_query.single_mut() else { return; };
    
    // Check if a unit is selected
    if let Some(_selected_entity) = selection.selected_unit {
        // Show panel
        *panel_visibility = Visibility::Visible;
        
        // TODO: Get unit info from BaseStats, CurrentStats, AttackProfile components
        *info_text = Text::new("Unit Selected\n\nDetails not available yet with new component system.\n\nTODO: Implement with BaseStats, CurrentStats, and AttackProfile components.");
    } else {
        // Hide panel when no unit selected
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