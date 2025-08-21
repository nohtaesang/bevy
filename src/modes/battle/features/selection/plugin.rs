

// =============================================
// src/modes/battle/features/selection/plugin.rs
// =============================================
use bevy::prelude::*;
use crate::app::schedule::Phase;
use crate::app::state::{AppState, ModeState};
use super::resources::{Selected, SelectionHighlightConfig};
use super::systems_input::select_on_left_click;
use super::systems_view::sync_selection_highlight;


pub struct SelectionPlugin;
impl Plugin for SelectionPlugin {
fn build(&self, app: &mut App) {
app.init_resource::<Selected>()
.init_resource::<SelectionHighlightConfig>()
.add_systems(
Update,
select_on_left_click
.in_set(Phase::Input)
.run_if(in_state(AppState::InGame).and(in_state(ModeState::Battle))),
)
.add_systems(
Update,
sync_selection_highlight
.in_set(Phase::ViewSync)
.run_if(in_state(AppState::InGame).and(in_state(ModeState::Battle))),
);
}
}


// =============================================
// NOTE: Register SelectionPlugin from your battle mode plugin
// use crate::modes::battle::features::selection::plugin::SelectionPlugin;
// app.add_plugins(SelectionPlugin);