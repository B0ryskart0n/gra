use std::collections::HashMap;

use super::components::*;
use bevy::prelude::*;

const DASH_TIME: f32 = 0.5;
const ENEMY_SPAWN_INTERVAL: f32 = 5.0;

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub direction: Vec3,
    pub dash: bool,
    pub attack: bool,
}
/// Timer of 1 second, scaled by the `Stats` `attack_speed`
#[derive(Resource)]
pub struct AttackTimer(pub Timer);
impl Default for AttackTimer {
    fn default() -> Self {
        AttackTimer(Timer::from_seconds(1.0, TimerMode::Once))
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
    pub fn item_stat(&self, item: &Item) -> f32 {
        *self.0.get(item).unwrap_or(&0u8) as f32 * Item::stat(item)
    }
}
