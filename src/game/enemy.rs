use super::*;
use bevy::prelude::*;

const ENEMY_SPEED: f32 = 4.0 * PIXELS_PER_METER;
const ENEMY_SIZE: f32 = 0.4 * PIXELS_PER_METER;
const ENEMY_HEALTH: f32 = 3.0;

pub fn spawn(time: Res<Time>, mut commands: Commands, mut q_spawners: Query<&mut EnemySpawner>) {
    q_spawners.iter_mut().for_each(|mut timer| {
        if timer.0.tick(time.delta()).is_finished() {
            commands.spawn((
                Enemy,
                Health(ENEMY_HEALTH),
                RigidBody::Dynamic,
                Collider::rectangle(ENEMY_SIZE, ENEMY_SIZE),
                Sprite::from_color(Color::srgb(1.0, 0.0, 0.6), Vec2::splat(ENEMY_SIZE)),
                Transform::from_translation(Vec3::from((320.0, 180.0, 0.5))),
                DespawnOnExit(MainState::Game),
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
                .normalize();
            transform.rotation = Quat::from_rotation_arc_2d(SPRITE_ORIENTATION, towards_player);
            v.0 = ENEMY_SPEED * towards_player;
        });

    Ok(())
}
pub fn hit(
    mut commands: Commands,
    mut q_enemies: Query<(Entity, &mut Health), With<Enemy>>,
    q_projectiles: Query<Entity, (With<Projectile>, Without<Enemy>)>,
    collisions: Collisions,
) {
    q_enemies.iter_mut().for_each(|(enemy, mut health)| {
        collisions
            .entities_colliding_with(enemy)
            .for_each(|entity| {
                // If an enemy gets hit with two projectiles it gets 2 damage;
                if q_projectiles.contains(entity) {
                    health.0 -= 1.0;
                    // One projectile can damage two enemies. This also leads to duplicate despawns.
                    commands.entity(entity).try_despawn();
                }
            })
    });

    // TODO Compare the two approaches

    q_projectiles.iter().for_each(|projectile| {
        for entity in collisions.entities_colliding_with(projectile) {
            if q_enemies.contains(entity) {
                // lower entity health by 1.0
                commands.entity(projectile).despawn();
                break;
            }
        }
    })
}
