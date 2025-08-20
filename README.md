src/
  app/
    state.rs            // AppState, ModeState::Battle/Management
    schedule.rs         // Phase: Input→Intent→Apply→Publish→ViewSync
    plugins.rs

  domain/               // ★ SSOT: 규칙/수식/인덱스/불변식 (UI 無)
    units/
      components.rs     // Unit, SquadId, Level, Xp, PerkSlots(★ 유닛에 붙는다)
      stats.rs          // Damage, Range, Move/AP, Crit 등 기본 스탯 컴포넌트
    perks/
      components.rs     // PerkId, PerkSlots 레이아웃(슬롯/배타/스택 구조)
      rules.rs          // 슬롯 규칙: 전파=3중1, OnHit=다중, Element=1, Traits/Pattern 플래그
      registry.rs       // 데이터 드리븐 정의(레벨별 수치, 설명 등)
      events.rs         // PerkAssignedApplied(확정 결과), PerkRemovedApplied 등
    combat/
      components.rs     // TeamId, AttackTag 등 코어 전투용 보조 컴포넌트
      attack.rs         // ① 기본 피해/치명타 공식
      statuses.rs       // ② Element(화염/빙결/전기/독), 쿨다운/해제
      projectile/
        onhit.rs        // ③ OnHit: Explode/Knockback/Gravity (정책: FinalOnly/EveryHit)
        propagation.rs  // ④ Propagation: Pierce/Ricochet/Chain + OriginPolicy
      pipeline.rs       // 처리 순서 고정: ①→②→③→④ (반복), attacker의 PerkSlots 사용
      events.rs         // DamageApplied, ElementApplied, OnHitApplied, Propagated …
    enemies/
      archetypes.rs     // Rush/Shielded/Exploder/Regenerator/Necromancer …
      traits.rs         // 저항/넉백불가/보스 태그
      resistances.rs    // 속성 내성/약점 테이블
    wave/
      scaler.rs         // 웨이브 난이도 곡선(HP/속도/스폰수)
      budget.rs         // 대군 스폰 포인트/조합 산출
      schedule.rs       // WaveSpec/BossWaveSpec, 시드
      events.rs         // WaveStarted/WaveEnded, EnemySpawnBudgeted
    map/
      components.rs     // GridPos, TerrainKind
      grid_index.rs     // 대량 점유/통행 인덱스(수백~수천 적 대응)
      flow_field.rs     // 대군 이동 최적화(옵션)

  infra/
    input/{keyboard.rs, mouse.rs}
    view_core/{theme.rs, z_index.rs, fonts.rs}
    profiling/{event_tap.rs, frame_stats.rs}

  modes/
    battle/
      plugin.rs         // run_if(in_state(Battle)); 기능 플러그인 등록
      features/
        squad_roster/   // ★ 유닛(1~6) 선택/전환 UI
          plugin.rs
          view.rs
          events.rs     // ActiveUnitChanged 등 (선택된 대상)
        perk_choice/    // ★ 웨이브 보상: “어느 유닛에 어떤 퍼크를 줄지” 선택
          plugin.rs
          coordinator.rs // WaveCleared → roll → (unit 선택 + perk 선택)
          events.rs      // PerkAssignRequested{unit, perk_id}
          systems.rs     // rules 검증→ PerkAssignedApplied(※ domain.perks.events)
          view.rs        // 유닛 초상 1~6 + 슬롯/제안 카드 UI + 미리보기
        firing_pattern/
          plugin.rs; coordinator.rs; events.rs; systems.rs; view.rs
        projectile_effects/
          plugin.rs; coordinator.rs; events.rs; systems.rs; view.rs
        propagation_slot/
          plugin.rs; coordinator.rs; events.rs; systems.rs; view.rs
        elemental_effects/
          plugin.rs; coordinator.rs; events.rs; systems.rs; view.rs
        elemental_synergy/
          plugin.rs; systems.rs; view.rs  // SynergyTriggeredApplied 구독→HUD/VFX
        wave_runtime/
          plugin.rs; coordinator.rs; systems.rs; view.rs
        spawner_horde/
          plugin.rs; systems.rs; view.rs  // 풀링·스폰 배치(성능)
        ai_update/
          plugin.rs; systems.rs           // 대군 이동/공격 tick-slice
        overlays/
          plugin.rs; view.rs              // 사거리/경로/전파 미리보기(경량, ViewSync)
        loot_supply/
          plugin.rs; systems.rs; view.rs
        boss_modifiers/
          plugin.rs; systems.rs; view.rs

    management/          // (선택) 전투 외 관리 화면
      plugin.rs
      features/
        loadout/
          plugin.rs; systems.rs; view.rs
        perk_planning/
          plugin.rs; coordinator.rs; systems.rs; view.rs


 
 
 
Phase 파이프라인 고정

Input → Intent → Apply → Publish → ViewSync

시스템은 반드시 한 Phase에만 등록해 순서/1프레임 이슈 차단.


이벤트 3단 규약

XxxRequested → XxxApplied → XxxUiSynced

나머지 상태 알림은 가능하면 Changed<T>로 처리.


뷰 쪽 “작은 코디네이터”

view/features/<feature>/coordinator.rs 하나 두고
여러 입력을 한 점에서 우선순위/상호배타 처리(팬아웃/레이스 방지).

코어(게임플레이)는 그대로 둠 = SSOT 유지.

이렇게만 해도 “이벤트가 많아져서 추적 어렵다”는 문제가 대부분 줄어듭니다.







바로 써먹는 요청 문장들

초기 세팅(Phase + 폴더 스캐폴드)

“하이브리드로 초기 세팅해줘: Phase 파이프라인( Input→Intent→Apply→Publish→ViewSync ) 넣고, domain/, modes/battle/features/, infra/, app/ 스캐폴드/코드 골격 생성해줘. 기존 로직 변경 없이 구조만 정리해.”

이벤트 3단 규약 도입

“이벤트 3단 규약 적용 패치 만들어줘: 현재 이벤트를 *Requested → *Applied → *UiSynced로 리팩터하고, 시스템을 Phase에 매핑하는 PR용 diff로 보여줘.”

뷰 코디네이터(교통정리 시스템) 추가

“기능 코디네이터 넣어줘: modes/battle/features/~~/coordinator.rs에서 여러 입력을 하나의 ~~Requested로 정규화하도록 만들어줘. 중복/우선순위/검증까지 포함.”

기존 기능을 슬라이스로 “승격”

“~~ 기능을 feature-slice로 승격해줘: modes/battle/features/~~ 폴더 만들고 plugin.rs, coordinator.rs, events.rs, systems.rs, view.rs 골격과 Phase 배치까지 셋업해줘. 코어 변경은 최소화.”

새로운 전투 규칙(도메인 코어) 추가

“도메인 코어에 규칙 추가해줘: domain/combat/projectile/onhit.rs에 Gravity {radius, force} 효과와 처리 순서 통합해줘. pipeline.rs에도 반영하고 단위 테스트 템플릿도 같이.”

퍼크/슬롯 시스템 연결

“퍼크 슬롯 기능 붙여줘: domain/perks에 슬롯/배타 규칙 정의하고, modes/management/features/perk_selection에서 UI·교체 흐름을 PropagationChosenRequested/Applied로 연결해줘.”

시너지 연출만 추가 (규칙은 유지)

“시너지 HUD/VFX 추가해줘: combat에서 발생하는 SynergyTriggeredApplied를 구독하는 modes/battle/features/elemental_synergy 만들고, 아이콘/텍스트/이펙트 ViewSync에 묶어줘.”

성능·게이팅

“기능 토글/게이팅 넣어줘: FeatureFlags 리소스로 ~~ 기능 on/off 가능하게 하고, 모든 시스템에 run_if(Mode/Flag) 달아줘.”

내 요청에 같이 적으면 좋은 정보(선택)

“Bevy 버전: 0.x”

“현재 폴더 트리(간단히)”

“대상 기능 이름 / 동작 목표 / 성공 기준(예: UI에서 버튼 클릭 → ~~Requested 발행)”

예시로 한 줄:

“하이브리드로 초기 세팅 + ‘propagation_slot’ 슬라이스 추가해줘. 관리 화면에서 관통/도탄/연쇄 중 1개 선택 → PropagationChosenRequested/Applied 흐름까지 Phase에 맞춰 구현.”

이런 식으로만 말해주면, 내가 바로 해당 구조/코드 골격까지 깔끔하게 만들어줄게.





Phase 배치(권장)

Input: 장치 입력 수집

Intent: squad_roster/perk_choice 코디네이터가 *Requested 발행

Apply: 도메인 규칙 적용(PerkSlots 갱신, attack 파이프라인 계산)

Publish: *Applied 결과 이벤트 방출(SSOT에서만)

ViewSync: HUD/오버레이/아이콘 갱신(최종 한 번)