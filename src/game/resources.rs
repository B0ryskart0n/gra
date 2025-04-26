use std::collections::HashMap;

use super::components::*;
use bevy::prelude::*;

const ATTACK_SPEED: f32 = 2.0;
const DASH_TIME: f32 = 0.5;
const ENEMY_SPAWN_INTERVAL: f32 = 5.0;

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub direction: Vec3,
    pub dash: bool,
    pub attack: bool,
}
#[derive(Resource)]
pub struct AttackSpeed(pub Timer);
impl Default for AttackSpeed {
    fn default() -> Self {
        AttackSpeed(Timer::from_seconds(1.0 / ATTACK_SPEED, TimerMode::Once))
    }
}
#[derive(Resource)]
pub struct DashTimer(pub Timer);
impl Default for DashTimer {
    fn default() -> Self {
        DashTimer(Timer::from_seconds(DASH_TIME, TimerMode::Once))
    }
}
#[derive(Resource)]
pub struct EnemySpawn(pub Timer);
impl Default for EnemySpawn {
    fn default() -> Self {
        EnemySpawn(Timer::from_seconds(ENEMY_SPAWN_INTERVAL, TimerMode::Repeating))
    }
}

#[derive(Resource, Default)]
pub struct PlayerEquipment(HashMap<Item, u8>);
impl PlayerEquipment {
    pub fn pickup(&mut self, item: Item) {
        self.0.entry(item).and_modify(|count| *count += 1).or_insert(1);
    }
    pub fn hud_nodes(&self, asset_server: Res<AssetServer>, spawner: &mut ChildSpawnerCommands) {
        self.0.iter().filter(|(_, val)| **val != 0u8).for_each(|(key, _)| {
            spawner.spawn(ImageNode::new(key.image(&asset_server)));
        });
    }
}
