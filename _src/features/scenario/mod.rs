//! Scenario system for different game levels
//!
//! This module contains different scenario plugins that set up
//! specific game levels with their own maps, units, and objectives.

pub mod level_1;

pub use level_1::Level1Plugin;