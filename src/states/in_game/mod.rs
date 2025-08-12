//! InGame state module
//!
//! This module contains all functionality related to the InGame state.

pub mod plugin;
pub mod states;

pub use plugin::InGamePlugin;
pub use states::{TurnState, SelectionState, UnitCommandState};