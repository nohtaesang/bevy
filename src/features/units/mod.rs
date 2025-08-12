//! Unit systems - ally and enemy units

pub mod ally;
pub mod enemy;
pub mod shared;
pub mod plugin;

// Re-export main types
pub use ally::Unit;
pub use enemy::Enemy;
pub use shared::{AttackDirection, AttackType, AttackRange};
pub use plugin::UnitsPlugin;