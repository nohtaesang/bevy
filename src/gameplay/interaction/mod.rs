pub mod plugin;
pub mod state;
pub mod events;
pub mod resources;            

pub mod systems {
    pub mod world_to_grid;
    pub mod hover;
    pub mod hotkeys;
    pub mod clicks;
    pub mod cancel;
}

pub use plugin::InteractionPlugin;
pub use state::PlayerIntent;
pub use resources::SelectionCtx;  // ⬅️ 여기서 re-export
pub use events::{TileHovered, TileClicked, HoverOutside, ClickOutside,
                 PlayerIntentChanged, SelectionChanged, CommandRequested};
