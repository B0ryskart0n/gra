use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Velocity(pub Vec3);

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
    pub fn stat(&self) -> f32 {
        match self {
            // Attack speed
            Self::Banana => 0.5,
        }
    }
}

#[derive(Component, Default)]
pub struct Pickable;

#[derive(Component)]
pub struct HealthHud;

#[derive(Component)]
pub struct EquipmentNode;
