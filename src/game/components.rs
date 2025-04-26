use bevy::prelude::*;

use super::resources::PlayerEquipment;

const PLAYER_SPEED: f32 = 120.0;
pub const PLAYER_HEALTH: f32 = 100.0;
const ATTACK_SPEED: f32 = 2.0;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Stats {
    pub max_health: f32,
    pub attack_speed: f32,
    pub movement_speed: f32,
}
impl Default for Stats {
    fn default() -> Self {
        Self {
            max_health: PLAYER_HEALTH,
            attack_speed: ATTACK_SPEED,
            movement_speed: PLAYER_SPEED,
        }
    }
}
impl Stats {
    pub fn apply_equipment(&mut self, eq: &PlayerEquipment) {
        self.attack_speed *= 1.0 + eq.item_stat(&Item::Banana);
    }
}

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
