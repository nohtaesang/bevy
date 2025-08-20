// src/infra/view_core/z_index.rs
use bevy::prelude::*;

/// Z 정렬 규칙:
/// - (보통) 카메라가 -Z 방향을 보므로, Z가 '작을수록' 화면 위쪽(앞)에 보입니다.
///   만약 너의 프로젝트에서 반대로 보인다면, 아래 값들의 부호만 전체적으로 뒤집어 써도 됩니다.
/// - 넉넉히 점프(10~100 단위)를 둬서 중간 레이어를 나중에 쉽게 끼워넣을 수 있게 해두세요.
#[repr(i16)]
#[derive(Clone, Copy, Debug)]
pub enum ZLayer {
    // === 월드(배경→전경) ===
    Background = -3000,  // 배경(패럴랙스 등)
    GroundDecal = -1100, // 지면 데칼
    Tiles = -1000,       // 타일(맵 기본면)
    GridOverlay = -950,  // 그리드 라인
    PropsStatic = -900,  // 나무/바위 등 정적 오브젝트
    Units = -800,        // 유닛 본체
    Shadows = -790,      // 그림자(유닛보다 약간 뒤)
    Projectiles = -700,  // 투사체
    Effects = -600,      // 파티클/히트 이펙트
    SelectionFx = -550,  // 선택 테두리/하이라이트
    DebugWorld = -500,   // 디버그용 월드 기즈모

    // === 오버레이(UI 아님, 월드에 붙는 것) ===
    RangePreview = -400, // 사거리 미리보기
    PathPreview = -390,  // 경로 미리보기

    // === UI 계층(별도 카메라 안 쓰는 경우) ===
    UiBase = -200,    // 일반 HUD
    UiTooltip = -150, // 툴팁
    CursorTop = -100, // 커서/드래그 가이드 최상단

    // === 최상단 디버그 ===
    DebugTop = -50,
}

impl ZLayer {
    #[inline]
    pub const fn z(self) -> f32 {
        self as i16 as f32
    }
    /// 같은 레이어 안에서 소폭 조정이 필요할 때(+/- 0~9 정도)
    #[inline]
    pub fn with_offset(self, dz: f32) -> f32 {
        self.z() + dz
    }
}

/// 필요하면 런타임에서 통째로 바이어스를 주는 옵션(모드/스킨 전환 등)
#[derive(Resource, Default)]
pub struct ZBias {
    pub world_bias: f32, // 예: 컷신에서 전체 월드를 뒤로 밀기 등
    pub ui_bias: f32,
}

impl ZBias {
    #[inline]
    pub fn z(&self, layer: ZLayer) -> f32 {
        let base = layer.z();
        match layer {
            ZLayer::UiBase | ZLayer::UiTooltip | ZLayer::CursorTop => base + self.ui_bias,
            _ => base + self.world_bias,
        }
    }
}
