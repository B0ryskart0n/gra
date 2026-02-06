use super::*;
use bevy::prelude::*;

const ENEMY_SPEED: f32 = 4.0;
const ENEMY_SIZE: f32 = 0.4;
const ENEMY_HEALTH: f32 = 3.0;

pub fn spawn(time: Res<Time>, mut commands: Commands, mut q_spawners: Query<&mut EnemySpawner>) {
    q_spawners.iter_mut().for_each(|mut timer| {
        if timer.0.tick(time.delta()).is_finished() {
            commands.spawn((
                Name::new("Enemy"),
                Enemy,
                Health(ENEMY_HEALTH),
                RigidBody::Dynamic,
                // Spawn at the spawner, so relative position = ZERO
                Transform::from_translation(Vec3::ZERO),
                Sprite::from_color(Color::srgb(1.0, 0.0, 0.6), Vec2::splat(ENEMY_SIZE)),
                Collider::rectangle(ENEMY_SIZE, ENEMY_SIZE),
                CollisionLayers::new(
                    CollisionGroup::Enemy,
                    [CollisionGroup::Player, CollisionGroup::Projectile],
                ),
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
// TODO Has non-zero probability of interaction with despawned entity.
pub fn hit(
    mut commands: Commands,
    mut q_enemies: Query<&mut Health, With<Enemy>>,
    q_projectiles: Query<Entity, With<Projectile>>,
    collisions: Collisions,
) {
    q_projectiles.iter().for_each(|projectile| {
        for colliding_entity in collisions.entities_colliding_with(projectile) {
            if let Ok(mut health) = q_enemies.get_mut(colliding_entity) {
                health.0 -= 1.0;
                commands.entity(projectile).despawn();
                break;
            }
        }
    })
}
