use bevy::prelude::*;

/// 매 프레임 막 눌린 키(디바운스된 just_pressed)
#[derive(Event, Debug, Clone, Copy)]
pub struct KeyJustPressed(pub KeyCode);

/// 커서 이동/클릭 이벤트는 그대로 유지
#[derive(Event, Debug, Clone, Copy)]
pub struct CursorMovedWorld { pub world: Vec2, pub screen: Vec2 }

#[derive(Event, Debug, Clone, Copy)]
pub struct CursorClickedWorld { pub world: Vec2, pub button: MouseButton }
