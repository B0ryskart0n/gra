use super::*;
use crate::utils::*;
use bevy::prelude::*;

const ENEMY_SPEED: f32 = 4.0 * PIXELS_PER_METER;
const ENEMY_SIZE: f32 = 0.4 * PIXELS_PER_METER;
const ENEMY_HEALTH: f32 = 3.0;

pub fn spawn(
    time: Res<Time<Fixed>>,
    mut commands: Commands,
    mut q_spawners: Query<(Entity, &mut EnemySpawner)>,
) {
    q_spawners.iter_mut().for_each(|(parent, mut timer)| {
        if timer.0.tick(time.delta()).is_finished() {
            commands.spawn((
                Enemy,
                Health(ENEMY_HEALTH),
                RigidBody::Dynamic,
                Collider::rectangle(ENEMY_SIZE, ENEMY_SIZE),
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
    mut q_enemies: Query<(&GlobalTransform, &mut Transform, &mut LinearVelocity), With<Enemy>>,
    q_player: Query<&GlobalTransform, (With<Player>, Without<Enemy>)>,
) -> Result {
    let player_pos = q_player.single()?.translation();
    q_enemies
        .iter_mut()
        .for_each(|(global_transform, mut transform, mut v)| {
            let towards_player = (player_pos - global_transform.translation())
                .xy()
                .normalize_or_zero();
            transform.rotation = Quat::from_rotation_arc_2d(SPRITE_ORIENTATION, towards_player);
            v.0 = ENEMY_SPEED * towards_player;
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
