// src/gameplay/units/spec.rs
use bevy::{asset::Asset, prelude::*, reflect::TypePath};
use serde::Deserialize;

use crate::gameplay::{
    tiles::components::GridPos,
    units::components::{
        ActionBudget, ActionTraits, AimDirs, Attack, ElementalImbue, Element, FiringMods, OnHitEffects,
        Propagation, Stats, TeamId, UnitBundle,
    },
};

#[derive(Asset, TypePath, Debug, Clone, Deserialize)]
pub struct UnitSpec {
    pub name: String,
    /// 팀은 프리셋에선 숫자(u8)로 관리(예: 0=ally, 1=enemy)
    pub team: u8,

    // 코어 스탯
    pub max_hp: i32,
    pub max_shield: i32,

    // 예산
    pub move_per_turn: u32,
    pub actions_per_turn: u32,

    // 전투/특성/발사/전파/OnHit/속성
    pub attack: AttackSpec,
    pub traits_: ActionTraitsSpec,
    pub firing: FiringModsSpec,        // multishot/fan 동시 사용 가능
    pub propagation: PropagationSpec,
    pub on_hit: OnHitEffectsSpec,
    pub imbue: ElementalImbueSpec,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum AimDirsSpec { Four, Eight }

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct AttackSpec {
    pub damage: i32,
    pub range: u32,
    pub crit_chance: f32,
    pub crit_mult: f32,
    pub dirs: AimDirsSpec,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ActionTraitsSpec {
    pub can_move_after_attack: bool,
    pub auto_reload: bool,
    pub can_phase: bool,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct FiringModsSpec {
    pub burst: u8,
    pub multishot_rays: u8,
    pub fan_rays: u8,
    pub lobbed: bool,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum PropagationSpec {
    None,
    Pierce { times: u8 },
    Ricochet { times: u8, branches: u8 },
    Chain { jumps: u8, radius: u8, diagonal: bool },
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ExplodeSpec { pub radius: u8, pub bonus_damage: i32, pub every_hit: bool }
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct KnockbackSpec { pub distance: u8, pub impact_damage: i32 }
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct GravityPullSpec { pub range: u8, pub strength: u8 }

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct OnHitEffectsSpec {
    pub explode: Option<ExplodeSpec>,
    pub knockback: Option<KnockbackSpec>,
    pub gravity: Option<GravityPullSpec>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum ElementSpec { Fire, Ice, Electric, Poison }

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ElementalImbueSpec {
    pub element: Option<ElementSpec>,
    pub cooldown_max: u32,
    pub cooldown: u32,
}

impl UnitSpec {
    /// 프리셋 → 런타임 번들로 변환
    pub fn to_bundle(&self) -> UnitBundle {
        UnitBundle {
            unit: crate::gameplay::units::components::Unit,
            team: TeamId(self.team),
            stats: Stats::new(self.max_hp, self.max_shield),
            move_budget: crate::gameplay::units::components::MoveBudget::new(self.move_per_turn),
            action_budget: ActionBudget::new(self.actions_per_turn),
            attack: Attack {
                damage: self.attack.damage,
                range: self.attack.range,
                crit_chance: self.attack.crit_chance,
                crit_mult: self.attack.crit_mult,
                dirs: match self.attack.dirs {
                    AimDirsSpec::Four => AimDirs::Four,
                    AimDirsSpec::Eight => AimDirs::Eight,
                },
            },
            traits_: ActionTraits {
                can_move_after_attack: self.traits_.can_move_after_attack,
                auto_reload: self.traits_.auto_reload,
                can_phase: self.traits_.can_phase,
            },
            firing: FiringMods {
                burst: self.firing.burst,
                multishot_rays: self.firing.multishot_rays,
                fan_rays: self.firing.fan_rays,
                lobbed: self.firing.lobbed,
            },
            propagation: match self.propagation {
                PropagationSpec::None => Propagation::None,
                PropagationSpec::Pierce { times } => Propagation::Pierce { times },
                PropagationSpec::Ricochet { times, branches } => Propagation::Ricochet { times, branches },
                PropagationSpec::Chain { jumps, radius, diagonal } => Propagation::Chain { jumps, radius, diagonal },
            },
            on_hit: OnHitEffects {
                explode: self.on_hit.explode.map(|e| crate::gameplay::units::components::Explode {
                    radius: e.radius, bonus_damage: e.bonus_damage, every_hit: e.every_hit
                }),
                knockback: self.on_hit.knockback.map(|k| crate::gameplay::units::components::Knockback {
                    distance: k.distance, impact_damage: k.impact_damage
                }),
                gravity: self.on_hit.gravity.map(|g| crate::gameplay::units::components::GravityPull {
                    range: g.range, strength: g.strength
                }),
            },
            imbue: ElementalImbue {
                element: self.imbue.element.map(|el| match el {
                    ElementSpec::Fire => Element::Fire,
                    ElementSpec::Ice => Element::Ice,
                    ElementSpec::Electric => Element::Electric,
                    ElementSpec::Poison => Element::Poison,
                }),
                cooldown_max: self.imbue.cooldown_max,
                cooldown: self.imbue.cooldown,
            },
            name: Name::new(self.name.clone()),
        }
    }
}

/// (옵션) 스폰 계획용 데이터
#[derive(Debug, Clone, Copy)]
pub struct UnitSpawnPlan {
    pub asset_path: &'static str, // 예: "units/ally.ron"
    pub at: GridPos,              // 어느 타일에 놓을지
}
