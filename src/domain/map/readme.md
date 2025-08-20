좋아. **domain/map**은 “맵과 점유 상태(누가 어느 칸을 쓰는가)”를 **SSOT(단일 진실 원천)**로 유지하는 도메인 계층이야. 뷰/엔티티 렌더/입력과 분리되고, 규칙·인덱스·불변식을 책임져.

역할 한 줄 요약

Map(지형): 크기(MapSize{w,h})와 타일 배열(TerrainKind[]) 보관.

OccupancyIndex(점유): w*h 크기의 Vec<Option<Entity>>로 “한 칸에 0~1개” 엔티티 점유 추적.

이벤트: MapInitRequested → (Apply에서 생성) → MapInitializedApplied → (ViewSync에서 렌더) → MapUiSynced.

파일 구성(도메인 전용)

components.rs

TerrainKind (예: Ground/Wall/…)

MapSize { w, h } (+ area())

Map { size, tiles } (길이 = w*h; 인덱싱만 제공, 렌더 좌표 모름)

grid_index.rs

GridPos { x, y } (도메인 좌표계, 정수 격자)

OccupancyIndex { size, slots: Vec<Option<Entity>> }

공개 API(반드시 O(1))

in_bounds(p) -> bool

is_free(p) -> bool

claim(p, e) -> bool (성공 시만 점유)

release(p)

try_move(from, to, e) -> bool (있으면 추천: 원자적으로 release→claim)

events.rs

MapInitRequested { size: MapSize }

MapInitializedApplied { size: MapSize }

MapUiSynced (뷰가 스폰/갱신 완료했음을 알림 — 선택)

plugin.rs

Startup: 한 번 MapInitRequested 송신(혹은 외부에서 보낼 수도 있음)

Apply: 요청 수신 시 Map/OccupancyIndex 생성(이미 있으면 무시/교체 정책)

Publish: MapInitializedApplied 방출

(Phase 붙인다면: Requested→Apply→Applied를 Phase::Apply/Publish에 배치)

불변식(깨지면 버그)

Map.tiles.len() == w*h

OccupancyIndex.slots.len() == w*h

claim()은 is_free()이고 경계 내일 때만 성공

하나의 타일엔 최대 1 엔티티(규칙) — 겹침은 상위 정책을 따로 두지 않는 한 금지

월드 좌표/픽셀 개념은 도메인에 없다 (그건 view 계층 책임)

Phase 배치(권장)

Input: (없음) 또는 MapInitRequested 트리거 UI/메뉴

Intent: (옵션) 중복 요청/리사이즈 정책 정리

Apply: Map/OccupancyIndex 생성·교체

Publish: MapInitializedApplied

ViewSync: 뷰 슬라이스(map_view)가 타일 스폰 → 끝나면 MapUiSynced(선택)

상호작용 포인트(후속 작업과 연결)

스폰(유닛/적): Apply에서 is_free(pos) → claim(pos, entity) 성공 시만 UnitSpawnApplied

이동: try_move(from, to, e)로 원자적 업데이트 → 성공 시 MoveApplied

제거/사망: release(pos) 후 엔티티 despawn

경로탐색/통행성: TerrainKind 별 is_passable_base() 같은 헬퍼를 도메인에 둔다(뷰 아님)

성능 팁(수백~수천 적 대비)

인덱싱: i = x + y*w (u32→usize 변환만)

대량 갱신 루프에서 slots는 연속 메모리라 CPU 캐시 효율 좋음

bulk API가 필요하면(예: 웨이브 리셋) clear_all()/release_many(&[GridPos]) 같은 일괄 함수 추가

지도 변경(크기 변동) 시 새 버퍼로 교체 후 이전 버퍼 Drop: 교체 타이밍은 Apply에서만