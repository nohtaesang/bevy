// src/gameplay/units/components.rs
use bevy::prelude::*;

/// 모든 유닛에 붙는 마커
#[derive(Component, Debug)]
pub struct Unit;

/// 진영/팀 식별자 (예: 아군=0, 적군=1)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TeamId(pub u8);

/// 체력/쉴드 (코어 스탯)
#[derive(Component, Debug, Clone, Copy)]
pub struct Stats {
    pub hp: i32,
    pub max_hp: i32,
    pub shield: i32,
    pub max_shield: i32,
}
impl Stats {
    pub fn new(max_hp: i32, max_shield: i32) -> Self {
        Self { hp: max_hp, max_hp, shield: max_shield, max_shield }
    }
    #[inline] pub fn is_dead(&self) -> bool { self.hp <= 0 }
    #[inline] pub fn clamp(&mut self) {
        self.hp = self.hp.clamp(0, self.max_hp);
        self.shield = self.shield.clamp(0, self.max_shield);
    }
}

/// 이동 거리(타일) 예산 — 턴마다 리셋
#[derive(Component, Debug, Clone, Copy)]
pub struct MoveBudget {
    pub current: u32,
    pub per_turn: u32,
}
impl MoveBudget {
    pub fn new(per_turn: u32) -> Self { Self { current: per_turn, per_turn } }
    #[inline] pub fn reset(&mut self) { self.current = self.per_turn; }
    #[inline] pub fn try_spend(&mut self, tiles: u32) -> bool {
        if self.current >= tiles { self.current -= tiles; true } else { false }
    }
}

/// 행동(AP) 예산 — 공격/장전/스킬 등에 사용, 턴마다 리셋
#[derive(Component, Debug, Clone, Copy)]
pub struct ActionBudget {
    pub current: u32,
    pub per_turn: u32,
}
impl ActionBudget {
    pub fn new(per_turn: u32) -> Self { Self { current: per_turn, per_turn } }
    #[inline] pub fn reset(&mut self) { self.current = self.per_turn; }
    #[inline] pub fn try_spend(&mut self, n: u32) -> bool {
        if self.current >= n { self.current -= n; true } else { false }
    }
}

/// 공격 가능한 방향(4방향 / 8방향)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AimDirs {
    Four,   // 상하좌우
    Eight,  // 대각 포함
}

/// 공격 기본값 (데미지/사거리/치명타/방향)
#[derive(Component, Debug, Clone, Copy)]
pub struct Attack {
    pub damage: i32,        // 기본 피해
    pub range: u32,         // 사거리(타일)
    pub crit_chance: f32,   // 0.0~1.0
    pub crit_mult: f32,     // 배수(예: 1.5 = 150%)
    pub dirs: AimDirs,      // 4방향 또는 8방향
}
impl Default for Attack {
    fn default() -> Self {
        Self { damage: 3, range: 3, crit_chance: 0.05, crit_mult: 1.5, dirs: AimDirs::Four }
    }
}
impl Attack {
    #[inline] pub fn allows_diagonal(&self) -> bool { matches!(self.dirs, AimDirs::Eight) }
}

/// 행동 특성(룰 변경) — 간단 플래그
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ActionTraits {
    /// 공격 후 이동 가능
    pub can_move_after_attack: bool,
    /// 자동 재장전
    pub auto_reload: bool,
    /// 적/장애물 통과 가능
    pub can_phase: bool,
}

/// 발사 모드 보정(연사/멀티샷/전방 난사/포물선)
///
/// 멀티샷과 전방 난사는 **동시에** 활성화될 수 있음.
/// - 멀티샷: 동일한 조준선에서 여러 발 동시 발사 (예: 3발)
/// - 전방 난사(Fan): 서로 다른 각도로 여러 조준선 생성 (예: 5방향)
#[derive(Component, Debug, Clone, Copy)]
pub struct FiringMods {
    /// 연사(추가 반복 횟수). 총 발사 횟수 = 1 + burst
    pub burst: u8,
    /// 멀티샷: 한 조준선에서 동시에 나가는 탄 수(0이면 단일)
    pub multishot_rays: u8,
    /// 전방 난사(부채꼴): 생성되는 조준선 수(0이면 비활성)
    pub fan_rays: u8,
    /// 포물선 사격(장애물 넘김 가능)
    pub lobbed: bool,
}
impl Default for FiringMods {
    fn default() -> Self {
        Self { burst: 0, multishot_rays: 0, fan_rays: 0, lobbed: false }
    }
}
impl FiringMods {
    #[inline] pub fn set_burst(&mut self, n: u8) { self.burst = n; }
    #[inline] pub fn set_multishot(&mut self, rays: u8) { self.multishot_rays = rays; }
    #[inline] pub fn set_fan(&mut self, rays: u8) { self.fan_rays = rays; }
    #[inline] pub fn total_projectiles_per_shot(&self) -> u32 {
        let mult = if self.multishot_rays == 0 { 1 } else { self.multishot_rays as u32 };
        let fan  = if self.fan_rays == 0 { 1 } else { self.fan_rays as u32 };
        mult * fan
    }
    #[inline] pub fn total_shots_per_action(&self) -> u32 {
        (1 + self.burst as u32) * self.total_projectiles_per_shot()
    }
}

/// (서로 배타, 3중 1) 전파 규칙 슬롯
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Propagation {
    None,
    /// 관통: 직선으로 n회 더 관통
    Pierce { times: u8 },
    /// 도탄: n회 도탄, 분기 수(예: 3방향)
    Ricochet { times: u8, branches: u8 },
    /// 연쇄: 인접 r타일 내 다른 적으로 n회 도약 (diagonal 허용 가능)
    Chain { jumps: u8, radius: u8, diagonal: bool },
}
impl Default for Propagation {
    fn default() -> Self { Propagation::None }
}

/// OnHit(명중 시 추가 효과) — 중첩 가능
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct OnHitEffects {
    pub explode: Option<Explode>,
    pub knockback: Option<Knockback>,
    pub gravity: Option<GravityPull>,
}
#[derive(Debug, Clone, Copy)]
pub struct Explode {
    pub radius: u8,         // +k
    pub bonus_damage: i32,  // +m
    /// true면 매 히트마다, false면 최종 명중에서만 1회
    pub every_hit: bool,
}
#[derive(Debug, Clone, Copy)]
pub struct Knockback {
    pub distance: u8,        // +k
    pub impact_damage: i32,  // +m (충돌 피해)
}
#[derive(Debug, Clone, Copy)]
pub struct GravityPull {
    pub range: u8,     // +k (영향 반경)
    pub strength: u8,  // 한 번에 당기는 칸 수 등
}

/// 속성 부여(유닛당 1개, 사용 후 쿨다운)
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ElementalImbue {
    pub element: Option<Element>,
    pub cooldown_max: u32,
    pub cooldown: u32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element { Fire, Ice, Electric, Poison }

/// 기본 유닛 번들 (뷰/스프라이트는 별도 레이어에서 관리)
#[derive(Bundle, Debug)]
pub struct UnitBundle {
    pub unit: Unit,
    pub team: TeamId,
    pub stats: Stats,
    pub move_budget: MoveBudget,
    pub action_budget: ActionBudget,
    pub attack: Attack,
    pub traits_: ActionTraits,
    pub firing: FiringMods,
    pub propagation: Propagation,
    pub on_hit: OnHitEffects,
    pub imbue: ElementalImbue,
    pub name: Name,
}