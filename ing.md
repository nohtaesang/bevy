좋아! 지금까지 만든 구조를 한 번에 파악·재사용할 수 있게 “다음 작업 시작 전에 나에게 줄 컨텍스트 문장”을 아래에 준비했어. 그대로 복붙하면 돼.

요약

상태/스케줄: ModeState(Battle/Management)로 모드 게이팅. 전역 Phase 체인(Input → Apply → Publish → ViewSync)을 사용.

도메인(SSOT): domain/map에 Map{size, tiles}, OccupancyIndex, 이벤트 3단 규약 도입(MapInitRequested → MapInitializedApplied → MapUiSynced(옵션)).

MapDomainPlugin: Phase::Input에서 1회 요청 → Apply에서 SSOT 생성 → Publish에서 Applied 발행. (Startup에서 이벤트 안 쏨)

뷰: modes/battle/features/map_view가 ViewSync에서 MapInitializedApplied 구독해 타일 렌더.

격자선은 라인 엔티티 없이 타일을 cell_size - tile_gap로 줄이고, 뒤에 언더레이 1장(grid_color)을 깔아 표현.

z는 infra/view_core/z_index::ZLayer로 중앙 관리.

카메라: infra/view_core/camera 모듈(입력/적용 분리, 의도 패턴).

input.rs: WASD/화살표 이동, Q/E 회전, 휠 줌, 중클릭/우클릭 드래그 팬 → CameraIntent에 누적.

apply.rs: 한 프레임에 일괄 적용(Transform/Projection). 드래그는 1px ≈ ortho.scale * pan_drag_multiplier로 자연스러운 속도.

plugin.rs: CamSet::Input → CamSet::Apply 체인, ModeState::Battle에서만 실행. Camera2d 스폰.

기타: CloseOnEsc 기본 동작 유의(원치 않으면 비활성화). 이벤트는 EventReader가 자기 첫 실행 이후 것만 읽으므로 Phaseに載せる設計を徹底。

다음에 나에게 줄 컨텍스트(그대로 복붙)

“우린 Bevy 0.16 스타일이고, 전역 Phase 체인(Input→Apply→Publish→ViewSync)과 이벤트 3단 규약(*Requested/*Applied/*UiSynced)을 사용해. ModeState::Battle로 모드 게이팅 중. 도메인 SSOT는 domain/map(Map/OccupancyIndex)이고 MapDomainPlugin이 Input에서 MapInitRequested 1회 → Apply에서 리소스 생성 → Publish에서 MapInitializedApplied를 내보내. 뷰는 modes/battle/features/map_view가 ViewSync에서 그 이벤트를 구독해 타일을 스폰하고, 언더레이+tile_gap 방식으로 격자선을 표현해. z 정렬은 infra/view_core/z_index::ZLayer로 중앙관리. 카메라는 infra/view_core/camera에 있고 의도 패턴으로 입력(CameraIntent)을 모아서 Apply 단계에서 한 번에 반영하며, 드래그 팬은 중/우클릭으로 받고 1px ≈ ortho.scale * pan_drag_multiplier로 변환해. 이 상태를 유지한 채로 다음 작업을 진행해줘.”