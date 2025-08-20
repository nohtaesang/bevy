use bevy::prelude::*;

/// 시스템을 한 Phase에만 등록하여 순서/레이스 이슈를 원천 차단
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Phase {
    Input,    // 장치 입력 수집 및 원자료 읽기
    Intent,   // 코디네이터가 요청 정규화(*Requested 발행)
    Apply,    // SSOT 변경(검증/계산/상태 갱신)
    Publish,  // *Applied 이벤트 방출(읽기 전용 파이프화)
    ViewSync, // 최종 뷰/오버레이 동기화
}

/// Phase 체인을 전역 Update 스케줄에 연결하는 플러그인
pub struct PhaseSchedulePlugin;

impl Plugin for PhaseSchedulePlugin {
    fn build(&self, app: &mut App) {
        // Update 단계에서 위 순서로 항상 실행
        app.configure_sets(
            Update,
            (
                Phase::Input,
                Phase::Intent,
                Phase::Apply,
                Phase::Publish,
                Phase::ViewSync,
            )
                .chain(),
        );
    }
}
