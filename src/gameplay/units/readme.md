좋아, units 모듈에 앞으로 “무엇이 더 들어오면 좋은지”를 큰 그림으로 정리해줄게. (코드 없음, 로드맵/파일 구조 아이디어만)

유닛 도메인 확장 체크리스트
1) 라이프사이클 & 턴 관리

턴 시작 리셋: MoveBudget, ActionBudget 초기화 시스템.

사망/제거 처리: Stats.is_dead() 감지 → DespawnRequested/UnitDied 이벤트 → 그리드 비움.

리바이브/소환/소멸: 특수 스킬/효과로 인한 생성·삭제 루틴.

2) 이동/경로

도달 가능 타일 계산: 이동력 기반 BFS/플러드 필.

경로 탐색: A* (대각 허용/불가, can_phase 반영).

실행기: CommandRequested::MoveTo → GridMove 발행 전 검증(경계, 점유, 코스트).

이동 애니메이션 타임라인: 보간/슬라이드, 이동 중 입력 잠금.

3) 공격/피해 판정

타깃팅 검증: 사거리, AimDirs(4/8방향), 시야/장애물(후속).

피해 공식: 기본 데미지 → 크리티컬 확률/배수 → 쉴드 우선 소모 → 체력 감소.

탄막 패턴: FiringMods(burst/multishot/fan/lobbed) 전개.

전파 규칙: Propagation(Pierce/Ricochet/Chain) 구현 순서 정의.

OnHit 효과: 폭발/넉백/중력, 충돌 데미지·벽 체크.

속성/쿨타임: ElementalImbue 부여 및 시너지 처리(이벤트 드리븐 권장).

4) 상태이상/버프 시스템

컴포넌트화된 스택/지속시간: 화염/빙결/전기/독 + 턴 감소.

시너지 트리거: 둘 이상 동시 적용 시 한 번성 효과 → 이벤트/파이프라인으로.

버프/디버프 프레임워크: 이동/공격/데미지에 훅을 꽂기 쉬운 레이어.

5) 선택/의도와의 연동(가드)

선택 유닛 유효성: TeamId가 플레이어 팀일 때만 Move/Attack 허용.

UI 피드백: 도달 가능 타일 하이라이트, 사거리/부채꼴 미리보기.

6) 데이터/프리셋 파이프라인 고도화

인덱스 파일: assets/units/index.ron(여러 유닛 일괄 스폰 계획).

프리셋 상속/오버라이드(후속): base + patch(선택).

핫리로드 대응: 스펙 변경 시 재스폰 or 런타임 값 갱신 전략.

7) 뷰/프리젠테이션(별 모듈과 연계)

스프라이트/메시/애니메이션: 팀 색상 틴트, 선택 링, 체력바.

디버그 기즈모: 사거리 원, 경로 라인, 도달 영역.

피해 숫자 팝업/이펙트: OnHit 시각화.

8) AI(후속)

간단 행동: 가장 가까운 적에게 이동→공격.

위협 평가/커버: 타일 가중치 지도.

행동 포인트 최적화: 이동/공격 순서 의사결정.

9) 이벤트/셋/순서

SystemSet 정리: UnitsSet::{Validate, Execute, Resolve, Cleanup} 등으로 파이프라인 명확화.

이벤트 정의: UnitDamaged, UnitHealed, ProjectileSpawned, KnockbackRequested 등.

10) 저장/로드 & 테스트

세이브 포맷: 유닛 상태 스냅샷(HP/버프/좌표/행동력).

단위 테스트: 데미지/크리/속성 시너지/경로 검증.

프로파일링 훅: 대규모 유닛 성능 체크.

추천 파일 구조(예시)
src/gameplay/units/
  components.rs
  resources.rs
  spec.rs
  assets.rs
  systems/
    movement.rs            // BFS/A*, MoveTo 실행기
    combat.rs              // 타깃팅/피해/크리/쉴드
    firing.rs              // burst/multishot/fan/lobbed 전개
    propagation.rs         // pierce/ricochet/chain
    onhit.rs               // 폭발/넉백/중력
    status.rs              // 속성/버프/시너지/쿨타임
    lifecycle.rs           // 스폰/사망/정리/턴 리셋
    debug_visual.rs        // 사거리/경로 기즈모
    guards.rs              // 선택/의도 가드(플레이어 팀)
    spawn_from_assets.rs   // 지금 만든 것
  plugin.rs

바로 다음 액션(우선순위)

TurnStart 리셋 시스템(Move/AP 초기화) → 전투 루프가 돌도록.

MoveTo 실행기(기본): 도달 가능 검사 + GridMove 발행 + 이동 코스트 차감.

Attack(기본): 사거리/방향 체크 + 데미지/크리/쉴드 처리.

(선택) 하이라이트 뷰: 이동/사거리 미리보기.

이렇게 가면 “맵에 유닛 올리고 → 선택 → 이동/공격”의 최소 전투 루프가 완성돼요. 이후에 패턴/전파/OnHit/속성/AI로 점층 확장하면 됩니다.