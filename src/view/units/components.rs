    use bevy::prelude::*;

    use crate::gameplay::units::components::TeamId;

    /// 유닛 스프라이트(자식 엔티티)에 붙는 마커
    #[derive(Component)]
    pub struct UnitSprite;

    /// 부모(Unit) ↔ 자식(UnitSprite) 링크를 부모 쪽에 보관
    #[derive(Component, Debug, Clone, Copy)]
    pub struct UnitSpriteLink(pub Entity);

    /// (확장 용) 뷰 관련 메타
    #[derive(Component, Clone, Copy, Debug)]
    pub struct UnitVisual {
        pub team: TeamId,
    }

    /// 0.16 스타일: Sprite 렌더에 필요한 모든 컴포넌트 포함
    #[derive(Bundle)]
    pub struct UnitSpriteBundle {
        pub sprite: Sprite,
        pub transform: Transform,
        pub global_transform: GlobalTransform,
        pub visibility: Visibility,
        pub inherited_visibility: InheritedVisibility,
        pub view_visibility: ViewVisibility,
        pub marker: UnitSprite,
        pub visual: UnitVisual,
    }

    impl UnitSpriteBundle {
        pub fn new(sprite: Sprite, transform: Transform, visual: UnitVisual) -> Self {
            Self {
                sprite,
                transform,
                global_transform: GlobalTransform::default(),
                visibility: Visibility::default(),
                inherited_visibility: InheritedVisibility::default(),
                view_visibility: ViewVisibility::default(),
                marker: UnitSprite,
                visual,
            }
        }
    }
