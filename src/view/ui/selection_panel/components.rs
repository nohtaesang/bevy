use bevy::prelude::*;

/// 선택된 유닛 정보를 담는 고정 패널의 루트
#[derive(Component, Debug)]
pub struct SelectionPanelRoot;

// ---- 패널 내부 위젯(텍스트) 마커 ----

#[derive(Component, Debug)]
pub struct SelNameText;

#[derive(Component, Debug)]
pub struct SelHpText;

#[derive(Component, Debug)]
pub struct SelShieldText;

#[derive(Component, Debug)]
pub struct SelMoveApText;

#[derive(Component, Debug)]
pub struct SelAttackText;
