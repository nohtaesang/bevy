// src/gameplay/units/mod.rs
pub mod components;
pub mod resources;
pub mod spec;
pub mod assets;
pub mod systems {
    pub mod spawn_from_assets;
}
pub mod plugin;

pub use plugin::UnitsPlugin;
