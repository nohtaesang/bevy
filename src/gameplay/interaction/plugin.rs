// src/gameplay/interaction/plugin.rs
use bevy::prelude::*;
use bevy::ecs::schedule::common_conditions::resource_exists;

use crate::app::state::AppState;
use crate::gameplay::tiles::prelude::{BaseTileMap, TileConfig};

use super::{
    events::{
        TileHovered, TileClicked, HoverOutside, ClickOutside,
        PlayerIntentChanged, SelectionChanged, CommandRequested,
    },
    resources::SelectionCtx, // ← SelectionCtx는 interaction의 resources에서
    systems::{
        world_to_grid::{world_cursor_to_grid_hover, world_click_to_grid_click},
        hover::apply_hover_to_ctx,
        hotkeys::handle_hotkeys_to_intent,
        clicks::handle_clicks_by_intent,
        cancel::cancel_on_right_click,  
    },
};

pub struct InteractionPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum InteractionSet {
    /// input(world) → grid pos 이벤트 변환
    MapInput,
    /// hover/핫키/클릭 해석
    Interpret,
}

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            // 리소스
            .init_resource::<SelectionCtx>()
            // 이벤트 타입 등록
            .add_event::<TileHovered>()
            .add_event::<TileClicked>()
            .add_event::<HoverOutside>()
            .add_event::<ClickOutside>()
            .add_event::<PlayerIntentChanged>()
            .add_event::<SelectionChanged>()
            .add_event::<CommandRequested>()
            // 세트 순서: MapInput → Interpret
            .configure_sets(
                Update,
                (InteractionSet::MapInput, InteractionSet::Interpret).chain(),
            )
            // 1) world→grid 이벤트 변환
            .add_systems(
                Update,
                (world_cursor_to_grid_hover, world_click_to_grid_click)
                    .in_set(InteractionSet::MapInput)
                    .run_if(in_state(AppState::Battle))
                    .run_if(resource_exists::<TileConfig>)
                    .run_if(resource_exists::<BaseTileMap>),
            )
            // 2) hover 적용 / 핫키 해석 / 클릭 해석
            .add_systems(
                Update,
                (cancel_on_right_click, apply_hover_to_ctx, handle_hotkeys_to_intent, handle_clicks_by_intent)
                    .in_set(InteractionSet::Interpret)
                    .run_if(in_state(AppState::Battle)),
            );
    }
}
