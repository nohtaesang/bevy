use bevy::prelude::*;
use crate::gameplay::tiles::components::GridPos;

///커서가 가리키는 타일(맵 안) 변경 시
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileHovered {
    pub pos: GridPos,
}

/// 타일 클릭(맵 안)
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileClicked {
    pub pos: GridPos,
}

// === 맵 밖 ===
#[derive(Event, Debug, Clone, Copy)]
pub struct HoverOutside {
    pub world: Vec2,   // 커서 월드좌표 (z는 0 가정)
}

#[derive(Event, Debug, Clone, Copy)]
pub struct ClickOutside {
    pub world: Vec2,
    pub button: MouseButton,
}


/// 의도(모드) 변경 알림
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerIntentChanged {
    pub prev: super::state::PlayerIntent,
    pub new_intent: super::state::PlayerIntent,
}

/// 셀렉션 변화(타일/유닛 등)
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionChanged {
    pub tile_changed: bool,
    pub unit_changed: bool,
}

/// 상호작용의 최종 산물: “명령 요청”
/// 실제 실행은 commands/executor에서!
/// 
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CommandRequested {
    /// 아직 유닛 결합 전이므로 위치만 보냄 (executor가 유닛/경로 확정)
    MoveTo { to: GridPos },
    /// 공격도 우선 타일 기준(대상 유닛 resolve는 executor/인덱스가 수행)
    AttackTile { at: GridPos },
    EndTurn,
}
