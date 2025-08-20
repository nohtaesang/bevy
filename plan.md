좋아, “아주 작은 단위”로 타일 → 배치(유닛/적) → 이동 → 공격 까지 찍는 로드맵을 딱 잘라서 브리핑해줄게. 아래 순서대로 진행하면 매 스텝마다 눈으로 확인 가능한 결과가 나오고, Phase/이벤트 3단 규약도 그대로 유지돼.

0. 부트스트랩(반나절 컷)
목표

Phase 파이프라인 세트 정의하고, Battle 모드에서만 돌도록 기본 플러그인 구성.

이벤트 3단 규약 템플릿 준비.

할 일

app/schedule.rs

SystemSet(Input/Intent/Apply/Publish/ViewSync) 선언

각 세트는 run_if(in_state(AppState::InGame).and_then(in_state(ModeState::Battle)))

app/plugins.rs

BattleAppPlugin가 modes/battle/plugin.rs를 등록

공용 이벤트 템플릿 추가

XxxRequested, XxxApplied, XxxUiSynced 3종을 events.rs들에 분리

완료 기준

빈 전투 화면에서 Phase 세트가 정상 진행(로그로 세트 순서 확인).

유닛/타일 없어도 앱 크래시 없이 실행.

1. 타일 맵(SSOT) + 그리드 인덱스 (첫 가시화)
목표

도메인에서 맵이 SSOT: 크기/지형/점유 인덱스는 domain/map에만 존재.

View는 Publish 이후 한 번에 스폰.

폴더 & 구성

domain/map/components.rs

GridPos { x, y }, TerrainKind, MapSize { w, h }

domain/map/grid_index.rs

OccupancyIndex (예: Vec<Option<Entity>> 크기 w*h)

API: is_free(pos), claim(pos, entity), release(pos)

domain/map/events.rs

MapInitializedApplied { size }

modes/battle/features/map_view/

plugin.rs, systems.rs, view.rs, events.rs(선택)

View는 MapInitializedApplied 구독 → 타일 스프라이트 일괄 생성

초기화

Apply: 스타트업에서 맵 리소스 세팅 → MapInitializedApplied Publish

ViewSync: 타일 스폰

Phase 배치

(없음) → Apply(맵 리소스 생성) → Publish(MapInitializedApplied) → ViewSync(타일 스폰)

완료 기준

20×12 같은 테스트 맵이 화면에 그리드로 보임(지형은 단색/체커보드면 충분).

콘솔에서 (x,y) → index 변환/역변환 테스트 통과.

2. 유닛/적 “배치” (선택/포지션만, AI 없음)
목표

유닛/적을 타일 위에 스폰하고, 그리드 점유가 SSOT에서 갱신됨.

ActiveUnit 1명 선택 상태 유지(간단한 roster).

폴더 & 구성

domain/units/components.rs

Unit, TeamId { Ally|Enemy }, GridPos, (초기 최소 스탯: Hp { current, max })

domain/units/events.rs

UnitSpawnRequested { team, pos }

UnitSpawnApplied { entity, team, pos }

modes/battle/features/spawner_debug/ (디버그 전용)

Input: 키(U=아군, I=적) + 마우스 포인터 타일 좌표 → UnitSpawnRequested 발행

Apply: grid_index.is_free(pos) 검증 → 엔티티 스폰(+ GridPos, TeamId, Hp) → 인덱스 claim → UnitSpawnApplied

ViewSync: 간단한 색상 스프라이트 스폰(아군 파랑, 적 빨강)

modes/battle/features/squad_roster/

ActiveUnit { entity } 리소스

Input: 숫자 1..6로 ActiveUnit 전환 → ActiveUnitChanged(Publish 전용 알림 이벤트)

ViewSync: 선택 유닛 하이라이트(테두리/원)

Phase 배치

Input(키/마우스→SpawnRequested, 숫자→ActiveUnitChangedRequested)

Intent(선택/중복 방지 코디네이터: 같은 타일에 연속 요청 차단)

Apply(스폰 확정 + 인덱스 claim)

Publish(UnitSpawnApplied, ActiveUnitChangedApplied)

ViewSync(스프라이트/하이라이트 스폰)

완료 기준

빈 맵에서 클릭 위치에 U/I로 아군/적 스폰 가능.

점유 중 타일에는 스폰 불가(콘솔 경고).

숫자키로 ActiveUnit 전환되며 하이라이트 변경.

3. 유닛 “이동” (경로 단순/턴 개념 없이 시작)
목표

ActiveUnit을 클릭한 타일로 이동시키고, 인덱스를 안전하게 업데이트.

처음엔 텔레포트/또는 한 칸 이동으로 시작 → 이후 경로탐색 확장.

폴더 & 구성

domain/units/events.rs

MoveRequested { unit, to }

MoveApplied { unit, from, to }

modes/battle/features/move_action/

Input: 우클릭 타일 → MoveRequested { active_unit, to }

Intent: 코디네이터에서 “자기 팀만 가능/사거리(이동 거리) 기본 1칸” 등 가벼운 규칙

Apply:

grid_index.is_free(to) 확인

release(from), claim(to) 원자적 업데이트

GridPos 변경 → MoveApplied

ViewSync: 트윈/보간 이동(없어도 됨. 최초엔 위치 스냅)

Phase 배치

Input → Intent(검증/중복 제거) → Apply(인덱스 & 컴포 변경) → Publish(MoveApplied) → ViewSync(좌표 반영/애니)

완료 기준

점유 충돌 없이 움직임.

같은 프레임 내 이중 이동 방지(코디네이터에서 중복 요청 드랍).

범위 밖 요청/벽 지형 요청 차단 로그 확인.

4. “공격” (단일 타깃, 기본 공식)
목표

사거리 검사 → 공격 요청 → 피해 계산 → HP 감소 → 사망 시 디스폰까지.

도메인/전투 규칙은 최소 공식부터: damage = base_damage (치명타/속성은 다음 단계)

폴더 & 구성

domain/combat/components.rs

Attack { base_damage: i32, min_range: i32, max_range: i32 }

domain/combat/attack.rs

calc_basic_damage(attacker, defender) -> i32

domain/combat/events.rs

AttackRequested { attacker, target }

DamageApplied { target, amount, hp_after }

UnitDiedApplied { entity }

modes/battle/features/attack_action/

Input: 좌클릭 적 타일(ActiveUnit 존재 시) → AttackRequested

Intent(coordinator):

타깃이 Enemy인지, 사거리 min..=max 내인지 검증

Apply:

calc_basic_damage → Hp 감소

hp<=0면 인덱스 release(pos) + 엔티티 디스폰 → UnitDiedApplied

피해 결과는 DamageApplied

ViewSync: 간단한 히트 플래시/데미지 텍스트(선택)

Phase 배치

Input → Intent(검증) → Apply(계산/HP 반영/디스폰/인덱스) → Publish(DamageApplied, UnitDiedApplied) → ViewSync(피드백)

완료 기준

사거리 밖 클릭 시 요청 무시(로그 안내).

공격으로 적 HP 감소가 보이고, 0 이하 시 즉시 제거 + 타일 비워짐.

5. 품질/안전 그물(추천)

프로파일 핀: infra/profiling/event_tap.rs로 각 Phase별 이벤트 카운터 로그(프레임당)

테스트 시나리오(수동)

맵 생성 → 타일 100개 이상 렌더

아군 1, 적 1 스폰 → 점유 인덱스 2칸 채움

아군 이동(빈 칸 OK, 점유 칸 NG)

적 클릭으로 공격 → HP 0 → 디스폰 → 인덱스 해제

경계조건

같은 프레임에 MoveRequested 2회 들어와도 코디네이터가 1개만 통과

Changed<GridPos>는 ViewSync에서만 사용(데이터-뷰 분리)

6. 다음 확장 포인트(선택)

이동: 경로탐색(4방/8방, 코스트), 이동 포인트(MP) 소모

공격: 치명타/속성/OnHit/Propagation 파이프라인(지금의 구조에 그대로 접합)

적 AI: ai_update 슬라이스로 tick-slice 도입(프레임 나눠서 대군 처리)

오버레이: 사거리/경로 미리보기(overlays 슬라이스, ViewSync 전용)

바로 사용 가능한 “요청 문장” (복사해서 붙여넣기)

타일 베이스 깔기

“도메인 SSOT로 맵 초기화부터 MapInitializedApplied → ViewSync 스폰까지 코드 골격 만들어줘. domain/map과 modes/battle/features/map_view로 나눠.”

유닛/적 스폰(디버그)

“spawner_debug 슬라이스 추가: 마우스 타일 + 키보드(U=아군/I=적)로 UnitSpawnRequested 발행, Apply에서 그리드 인덱스 갱신 후 UnitSpawnApplied, ViewSync로 색상 스프라이트까지.”

이동

“move_action 슬라이스 추가: 우클릭 타일 → MoveRequested → 인덱스 release/claim → MoveApplied → ViewSync로 위치 반영. 같은 프레임 중복 요청은 코디네이터에서 드랍.”

공격

“attack_action 슬라이스 추가: 좌클릭 타깃 → AttackRequested → 기본 공식 → DamageApplied/UnitDiedApplied → ViewSync 히트 플래시. 사거리 검증은 Intent에서.”

최소 데이터 스펙(초안)

Hp { current: i32, max: i32 }

Attack { base_damage: i32, min_range: i32, max_range: i32 } (아군에만 붙여도 OK)

TeamId { Ally, Enemy }

GridPos(도메인 SSOT), WorldPos(뷰 계산 전용)

MapSize { w, h }, OccupancyIndex(Vec<Option<Entity>>) with (x + y*w) 인덱싱

원하면, 위 1단계부터 바로 **코드 골격(파일/모듈/플러그인 + 시스템 시그니처)**까지 생성해서 붙여줄게.
다음 메시지에 “1단계 타일 베이스부터 생성”이라고만 적어줘. 그러면 너의 폴더 구조에 맞춘 샘플 코드로 바로 깔끔하게 넣어줄게.