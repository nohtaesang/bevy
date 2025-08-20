use bevy::prelude::*;

use crate::app::{
    schedule::PhaseSchedulePlugin,
    state::{AppState, ModeState},
};

/// 앱 공용 플러그인 모음. (Step 1: 상태/스케줄만)
pub struct AppPlugins;

impl Plugin for AppPlugins {
    fn build(&self, app: &mut App) {
        app
            // Phase 파이프라인
            .add_plugins(PhaseSchedulePlugin)
            // 전역 상태 초기화
            .init_state::<AppState>()
            .init_state::<ModeState>()
            // 데모를 위해 기본 상태를 지정(원하면 main.rs에서 바꿀 수 있음)
            .insert_state(AppState::InGame)
            .insert_state(ModeState::Battle);

        // 주의: 실제 기능(타일/유닛/공격)은 이후 단계에서 별도 플러그인으로 추가합니다.
        // 모든 기능 시스템은 반드시 Phase의 한 구간에만 등록하세요.
    }
}
