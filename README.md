input + cursor → tile hit-test

goal: robust mouse/keyboard input that resolves to a GridPos.

ship: CursorWorldPos, HoveredTile(GridPos), hotkeys (1=move, 2=attack).

types/events: TileHovered(GridPos), TileClicked(GridPos).

why: every system (selection, UI, overlays) depends on this.

selection context (player-side only)

goal: single source of truth for “what’s selected” and “current intent”.

ship: SelectionCtx { selected_tile, selected_unit, intent: Idle|Move|Attack }.

types/events: SelectionChanged, IntentChanged.

why: prevents spaghetti between UI and gameplay.

grid index (spatial cache) + occupancy rules

goal: fast queries on big maps; enforce “one unit per tile”.

ship: GridIndex { unit_at[x,y], block_mask, move_cost }.

types/events: UnitSpawned, UnitDespawned, TileBlockedChanged, TileCostChanged, TileMoved.

why: movement, pathfinding, and AI need O(1) lookups.

pathfinding service (movement costs + reachability)

goal: reusable A*/Dijkstra service decoupled from UI.

ship: Pathfinder resource; API: shortest_path(start, goal), reachable(start, mp).

types/events: PathRequest/PathResponse.

why: unlocks movement preview, range overlays, AI.

overlays: hover, move range, attack range, path preview

goal: instant player feedback with zero game-state mutation.

ship: overlay renderer that consumes SelectionCtx + Pathfinder outputs.

types/events: OverlayRequest (range/path), internal batched mesh for performance.

why: lets you playtest interactivity before real actions exist.

turn/state machine (game + player substates)

goal: deterministic flow: GameState(InGame) → TurnState(Player|Enemy) → PlayerSubState(Idle|UnitSelected|Move|Attack...).

ship: states, transitions, guards (e.g., no action when animations running).

types/events: TurnBegan(Team), TurnEnded(Team).

why: everything gets simpler when transitions are formalized.

unit domain: components + spawn/despawn + action points

goal: minimal unit model to move and act.

ship: Unit, Team, ActionPoints, BaseStats; spawn helpers; serialization seeds.

types/events: UnitCreated, UnitKilled, APChanged.

why: you can now “play” a skeleton of the game loop.

command system (the one place that mutates gameplay)

goal: unify all game mutations behind commands for undo/logging/replay.

ship: Command enum (Move, Attack, Wait, EndTurn), CommandQueue, CommandResult.

types/events: CommandRequested, CommandCommitted, CommandFailed.

why: isolates rules from input/UI; enables replays & AI scripting.

movement execution + animation bridge

goal: authoritative movement with interpolation+blocking.

ship: MovementSystem (validates via GridIndex, reserves/commits tiles), MoveAnimation (non-blocking but state-aware).

types/events: MovementStarted, MovementFinished.

why: lets you test end-to-end: click → path → move → AP decrement.

combat core (hit calc, damage, death)

goal: deterministic, seedable combat with minimal stats.

ship: AttackResolver (range check, LOS if needed, crit, armor later), DamageSystem, DeathSystem.

types/events: AttackRequested, AttackResolved, DamageApplied, UnitDied.

why: you now have a playable loop: move/attack/end turn.

enemy AI (phase-based; uses same commands)

goal: simple, reliable AI that issues Commands (no UI coupling).

ship: behavior pipeline: perceive → choose target → pathfind → command queue.

types/events: AIThinkTick, AIPlanCommitted.

why: validates your command layer and pathfinder under load.

effects/status & terrain interactions (extensible buffs/debuffs)

goal: non-permanent modifiers without touching base stats.

ship: StatusEffect components, tick/expiry system, terrain auras (e.g., forest: +evasion; road: -move cost).

types/events: EffectApplied/Expired, TileAuraEntered/Left.

why: adds depth without destabilizing core rules.

FoW / visibility (optional but impactful)

goal: per-team visibility + discovery.

ship: VisibilityMap per team; reveal/hide overlays; AI gated by vision.

types/events: VisibilityChanged.

why: affects targeting, UI, and performance—good to add after basics work.

UI HUD & panels (action bar, unit tooltip, turn banner)

goal: clear, minimal UI bound to SelectionCtx + TurnState.

ship: action buttons (also hotkeys), unit card, combat preview tooltip.

types/events: UIClick(Action), Hotkey(Action).

why: polish + faster iteration; keep UI strictly read-from-state, write-as-commands.

save/load + deterministic replay

goal: reproducible sessions.

ship: snapshot of RNG seed, map, units, command log; replay runner.

why: huge for debugging and balancing.

content/data-driven config

goal: tune without recompiling.

ship: TOML/JSON for units, weapons, tileset; loader → components.

why: accelerates balancing + experimentation.

performance pass

goal: scale to large maps.

ship: sprite/mesh batching by Z/layer, chunked overlays, pooled entities, fixed-timestep for simulations, event coalescing.

why: you mentioned large-scale battles—lock this in before content bloat.

debug & tooling

goal: fast diagnosis.

ship: debug draw for grids/paths/LOS, stat overlay, event inspector, lag spike recorder.






다음 단계(선택)

SelectionCtx에 reachable/pending_path 필드 추가 후, 별도 previews.rs에서 Pathfinder/Index를 읽어 갱신 → 뷰 오버레이로 즉시 피드백.

clicks.rs에서 실제 유닛/적 선택 로직 추가(그리드 인덱스 참조).

commands 모듈을 만들어 CommandRequested를 검증/커밋하고, 이동/공격/사망/애니메이션 이벤트를 발행.

필요하면 바로 이어서 previews(이동범위/경로 미리보기) 스캐폴딩까지 만들어줄게.