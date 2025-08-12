//! Game resources
//!
//! This module contains all game resources including configuration,
//! shared state, and other data that needs to be accessed across systems.

pub mod tile_config;
pub mod tile_map;

pub use tile_config::TileConfig;
pub use tile_map::{TileMap, TileContent};