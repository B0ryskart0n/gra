use super::ENEMY_HEALTH;
use super::ENEMY_SIZE;
use super::ENEMY_SPEED;
use super::Enemy;
use super::EnemySpawner;
use super::Health;
use super::Player;
use super::Projectile;
use super::Velocity;
use crate::utils::*;
use bevy::prelude::*;

pub fn spawn(
    time: Res<Time<Fixed>>,
    mut commands: Commands,
    mut q_spawners: Query<(Entity, &mut EnemySpawner)>,
) {
    q_spawners.iter_mut().for_each(|(parent, mut timer)| {
        if timer.0.tick(time.delta()).finished() {
            commands.spawn((
                Enemy,
                Health(ENEMY_HEALTH),
                Velocity(Vec2::ZERO),
                Sprite::from_color(
                    Color::srgb(1.0, 0.0, 0.6),
                    Vec2::from((ENEMY_SIZE, ENEMY_SIZE)),
                ),
                Transform::from_translation(Vec3::from((320.0, 180.0, 0.5))),
                ChildOf(parent),
            ));
        }
    });
}
pub fn handle_state(
    mut q_enemies: Query<(&GlobalTransform, &mut Velocity), With<Enemy>>,
    q_player: Query<&GlobalTransform, (With<Player>, Without<Enemy>)>,
) -> Result {
    let player_pos = q_player.single()?.translation();
    q_enemies.iter_mut().for_each(|(t, mut v)| {
        v.0 = ENEMY_SPEED * (player_pos - t.translation()).xy().normalize_or_zero()
    });

    Ok(())
}
pub fn hit(
    mut commands: Commands,
    mut enemies: Query<(&mut Health, &GlobalTransform), With<Enemy>>,
    projectiles: Query<(Entity, &GlobalTransform), (With<Projectile>, Without<Enemy>)>,
) {
    projectiles
        .iter()
        .for_each(|(projectile, projectile_transform)| {
            for (mut health, enemy_position) in enemies.iter_mut() {
                if square_collide(
                    enemy_position.translation(),
                    ENEMY_SIZE,
                    projectile_transform.translation(),
                    0.0,
                ) {
                    health.0 = health.0 - 1.0;
                    commands.entity(projectile).despawn();
                    // Projectile despawned, so can't influence other enemies. Go to next projectile.
                    // Maybe if in the future projectiles can pass through then handle differently.
                    break;
                }
            }
        })
}
pub fn despawn_unhealthy(mut commands: Commands, query: Query<(Entity, &Health), With<Enemy>>) {
    query.iter().for_each(|(e, h)| {
        if h.0 <= 0.0 {
            commands.entity(e).despawn();
        }
    })
}
