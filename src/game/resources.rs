use std::collections::HashMap;

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
pub struct PlayerEquipment(HashMap<ItemType, u8>);
impl PlayerEquipment {
    fn pickup(&mut self, item: ItemType, mut pickup_events: EventWriter<ItemPickup>) {
        self.0.entry(item).and_modify(|count| *count += 1).or_insert(1);
        pickup_events.send_default();
    }
}

#[derive(Event, Default)]
struct ItemPickup;

#[derive(PartialEq, Eq, Hash)]
enum ItemType {
    Banana,
}
