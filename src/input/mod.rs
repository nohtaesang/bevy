// 하위 모듈 선언
pub mod plugin;
pub mod events;
pub mod resources;
pub mod systems;

// 바깥에서 쓸 때 편리하게 re-export
pub use plugin::InputPlugin;
pub use events::{CursorMovedWorld, CursorClickedWorld, KeyJustPressed};
pub use resources::CursorWorldPos;
