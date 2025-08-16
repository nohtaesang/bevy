//! Click handling systems
//!
//! Systems that detect clicks and classify what was clicked

use bevy::prelude::*;
use crate::features::tiles::{
    core::{TileConfig, world_to_tile_coords, GridIndex, Team},
    selection::SelectionCtx,
    interaction::{MovementValidation, AttackValidation},
    interaction::events::{TileClicked, ClickTarget, ClickTargetEvent},
};

/// Emit TileClicked event when user clicks
pub fn emit_tile_clicked(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    cfg: Res<TileConfig>,
    mut ev: EventWriter<TileClicked>,
) {
    // Only process left clicks for now
    if !mouse.just_pressed(MouseButton::Left) { 
        return; 
    }
    
    let Ok(win) = windows.get_single() else { return; };
    let Some(cursor) = win.cursor_position() else { return; };
    let Ok((cam, xf)) = cameras.get_single() else { return; };
    let Ok(world_pos) = cam.viewport_to_world_2d(xf, cursor) else { return; };

    // Convert to tile coordinates if on grid
    let tile_pos = world_to_tile_coords(world_pos, &cfg)
        .map(|(x, y)| IVec2::new(x, y));
    
    ev.send(TileClicked { 
        tile_pos,
        world_pos,
        button: MouseButton::Left,
    });
}

/// Classify what was clicked based on GridIndex and overlay states
pub fn classify_click_target(
    mut ev_in: EventReader<TileClicked>,
    mut ev_out: EventWriter<ClickTargetEvent>,
    grid: Res<GridIndex>,
    selection: Res<SelectionCtx>,
    movement_validation: Res<MovementValidation>,
    attack_validation: Res<AttackValidation>,
) {
    for event in ev_in.read() {
        // Handle outside grid clicks
        let Some(pos) = event.tile_pos else {
            ev_out.send(ClickTargetEvent(ClickTarget::OutsideGrid));
            continue;
        };
        
        if !grid.is_in_bounds(pos) {
            ev_out.send(ClickTargetEvent(ClickTarget::OutsideGrid));
            continue;
        }

        // Priority 1: Check overlays first (highest priority)
        if movement_validation.is_valid_move(pos) {
            ev_out.send(ClickTargetEvent(ClickTarget::MovementOverlay(pos)));
            continue;
        }
        
        if attack_validation.is_valid_attack(pos) {
            ev_out.send(ClickTargetEvent(ClickTarget::AttackOverlay(pos)));
            continue;
        }

        // Priority 2: Check for units/enemies
        if let Some(entity) = grid.unit_at(pos) {
            match grid.team_at(pos) {
                Some(Team::Player) => {
                    // Check if it's the currently selected unit
                    if Some(entity) == selection.selected_unit {
                        ev_out.send(ClickTargetEvent(ClickTarget::SelfUnit(pos, entity)));
                    } else {
                        ev_out.send(ClickTargetEvent(ClickTarget::FriendlyUnit(pos, entity)));
                    }
                }
                Some(Team::Enemy) => {
                    ev_out.send(ClickTargetEvent(ClickTarget::Enemy(pos, entity)));
                }
                None => {
                    // Shouldn't happen, but handle gracefully
                    warn!("Entity at {:?} has no team", pos);
                    ev_out.send(ClickTargetEvent(ClickTarget::EmptyTile(pos)));
                }
            }
            continue;
        }

        // Priority 3: Empty tile
        ev_out.send(ClickTargetEvent(ClickTarget::EmptyTile(pos)));
    }
}
