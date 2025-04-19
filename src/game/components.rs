use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct Player;

#[derive(PartialEq, Eq, Default, Component)]
pub enum PlayerState {
    #[default]
    Idle,
    Dashing,
    Attacking,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Lifetime(pub Timer);

#[derive(Component, PartialEq, Eq, Hash, Clone)]
#[require(Pickable)]
pub enum Item {
    Banana,
}
impl Item {
    pub fn image(&self, asset_server: &Res<AssetServer>) -> Handle<Image> {
        match self {
            Self::Banana => asset_server.load("banana.png"),
        }
    }
}

#[derive(Component, Default)]
pub struct Pickable;

#[derive(Component)]
pub struct HealthHud;

#[derive(Component)]
pub struct EquipmentNode;
