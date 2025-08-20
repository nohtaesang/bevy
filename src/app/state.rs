use bevy::prelude::*;

/// 전체 앱의 큰 상태(타이틀/인게임 등). Step 1에서는 InGame만 사용.
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    InGame,
}

/// 인게임 모드(전투/관리). 모드별로 시스템을 분리하기 위해 사용.
/// 전투/관리 모드 전환용 하위 상태
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum ModeState {
    /// 기본 시작 모드(원하면 바꿔도 됨)
    #[default]
    Battle,
    Management,
}
