use bevy::prelude::*;

/// 최신 프레임의 커서 위치(스크린/월드) 캐시
#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct CursorWorldPos {
    pub screen: Option<Vec2>,
    pub world: Option<Vec2>,
}
