use bevy::prelude::*;
use crate::app::schedule::Phase;
use crate::app::state::{AppState, ModeState};
use super::resources::{HoverTile, HoverHighlightConfig};
use super::systems_input::update_hover_tile;
use super::systems_view::sync_hover_highlight;

pub struct HoverTilePlugin;
impl Plugin for HoverTilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoverTile>()
          .init_resource::<HoverHighlightConfig>()
          .add_systems(
              Update,
              update_hover_tile
                  .in_set(Phase::Input)
                  .run_if(in_state(AppState::InGame).and(in_state(ModeState::Battle))),
          )
          .add_systems(
              Update,
              sync_hover_highlight
                  .in_set(Phase::ViewSync)
                  .run_if(in_state(AppState::InGame).and(in_state(ModeState::Battle))),
          );
    }
}
