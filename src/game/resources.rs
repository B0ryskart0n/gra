use bevy::prelude::*;

const ENEMY_SPAWN_INTERVAL: f32 = 5.0;

#[derive(Resource)]
pub struct EnemySpawn(pub Timer);
impl Default for EnemySpawn {
    fn default() -> Self {
        EnemySpawn(Timer::from_seconds(ENEMY_SPAWN_INTERVAL, TimerMode::Repeating))
    }
}
