//! Hover information UI component
//!
//! Displays information about the tile and unit under the mouse cursor

use bevy::prelude::*;
use crate::{
    features::tiles::{TileConfig, TileMap, world_to_tile_coords, TileContent, Team, units::bundles::UnitMarker},
};

/// Marker component for the hover info UI container
#[derive(Component)]
pub struct HoverInfoContainer;

/// Marker component for the hover info text
#[derive(Component)]
pub struct HoverInfoText;

/// System to setup the hover information UI
pub fn setup_hover_info_ui(mut commands: Commands) {
    // Create the hover info container
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                width: Val::Px(250.0),
                height: Val::Auto,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            BorderRadius::all(Val::Px(5.0)),
            HoverInfoContainer,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Hover over tiles for info"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                HoverInfoText,
            ));
        });
}

/// System to update hover information based on mouse position
pub fn update_hover_info(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    tile_config: Res<TileConfig>,
    tile_map: Res<TileMap>,
    _unit_query: Query<&Team, With<UnitMarker>>,
    mut text_query: Query<&mut Text, With<HoverInfoText>>,
) {
    let Ok(window) = windows.single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else {
        // Hide info when cursor is not in window
        if let Ok(mut text) = text_query.single_mut() {
            **text = "No hover information".to_string();
        }
        return;
    };
    
    let Ok((camera, camera_transform)) = camera_q.single() else { return; };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return; };
    
    let Some(tile_coords) = world_to_tile_coords(world_pos, &tile_config) else {
        if let Ok(mut text) = text_query.single_mut() {
            **text = "Outside game area".to_string();
        }
        return;
    };
    
    let tile_pos: IVec2 = tile_coords.into();
    
    // Build hover information string
    let mut info_text = format!("Tile: ({}, {})\n", tile_pos.x, tile_pos.y);
    
    // Check what's at this tile
    match tile_map.get_content(tile_pos) {
        TileContent::Unit(entity) => {
            info_text.push_str("Contents: Ally Unit\n");
            // TODO: Get unit info from proper components and display
            info_text.push_str("Unit details not available yet");
        }
        TileContent::Enemy(entity) => {
            info_text.push_str("Contents: Enemy Unit\n");
            // TODO: Get enemy info from proper components and display
            info_text.push_str("Enemy details not available yet");
        }
        TileContent::Empty => {
            info_text.push_str("Contents: Empty");
        }
        TileContent::Obstacle => {
            info_text.push_str("Contents: Obstacle");
        }
    }
    
    // Update the text
    if let Ok(mut text) = text_query.single_mut() {
        **text = info_text;
    }
}

/// System to cleanup hover info UI
pub fn cleanup_hover_info_ui(
    mut commands: Commands,
    query: Query<Entity, With<HoverInfoContainer>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}