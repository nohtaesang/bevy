use bevy::prelude::*;
use crate::app::state::AppState;
use super::{
  events::{CursorMovedWorld, CursorClickedWorld, KeyJustPressed},
  resources::CursorWorldPos,
  systems::{update_cursor_world_pos, emit_clicks_world, emit_key_just_pressed},
};

pub struct InputPlugin;
impl Plugin for InputPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<CursorWorldPos>()
      .add_event::<CursorMovedWorld>()
      .add_event::<CursorClickedWorld>()
      .add_event::<KeyJustPressed>()
      .add_systems(PreUpdate, update_cursor_world_pos.run_if(in_state(AppState::Battle)))
      .add_systems(Update, (emit_clicks_world, emit_key_just_pressed).run_if(in_state(AppState::Battle)));
  }
}
