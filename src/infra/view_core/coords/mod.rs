// src/infra/view_core/coords/mod.rs
//! 공통 그리드↔월드 좌표 유틸리티
//!
//! - 이 모듈 하나로 hover/selection/spawner/move_range 등이 동일 좌표계를 공유합니다.
//! - 카메라 회전/줌/팬은 "스크린→월드 변환" 쪽(카메라 시스템)에서 처리하고,
//!   여긴 순수하게 월드좌표와 그리드의 상호변환만 담당합니다.

pub mod plugin;
pub mod systems;
pub mod types;

// Re-exports for convenient access
pub use plugin::{CoordsPlugin, CoordsSet};
pub use systems::{grid_geometry_ready, sync_grid_geometry};
pub use types::{make_grid_geometry, GridGeometry};
