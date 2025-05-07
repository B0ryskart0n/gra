mod components;
mod hud;
mod pause;
mod player;
mod resources;

use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use std::cmp::Ordering;

use crate::CursorPosition;
use crate::MainState;
use crate::utils;
use components::*;
use player::*;
use resources::*;
use utils::*;

// TODO Shouldn't all sizes be whole number?
const INTERACTION_DISTANCE: f32 = 30.0;
const ENEMY_SIZE: f32 = 15.0;
const ENEMY_HEALTH: f32 = 3.0;
const ENEMY_SPEED: f32 = 100.0;
/// Rate of exponential decay in the distance between camera and it's goal.
const CAMERA_SPEED: f32 = 8.0;
const CURSOR_CAMERA_INFLUENCE: f32 = 0.3;

pub fn game_plugin(app: &mut App) {
    app.add_sub_state::<GameSubState>()
        // Since I want to rely on `resource_changed` condition I need to initiate
        // those resources at the top level instead of `OnEnter(GameState::Game)`.
        .init_resource::<PlayerInput>()
        .init_resource::<DashTimer>()
        .init_resource::<AttackTimer>()
        .init_resource::<EnemySpawn>()
        .init_resource::<PlayerEquipment>()
        .add_state_scoped_event::<PlayerDeath>(MainState::Game)
        .add_state_scoped_event::<ItemPickup>(MainState::Game)
        .add_systems(
            OnEnter(MainState::Game),
            (
                utils::reset_camera,
                spawn,
                player::spawn,
                hud::spawn,
                pause::spawn_invisible_overlay,
            ),
        )
        .add_systems(
            RunFixedMainLoop,
            player::handle_input
                .run_if(in_state(MainState::Game))
                .in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
        )
        .add_systems(
            FixedUpdate,
            (
                spawn_enemy,
                player_hit,
                (enemy_hit, despawn_unhealthy).chain(),
                player::attack,
                utils::lifetime,
                (player::handle_state, enemy_state, physics).chain(),
            )
                .run_if(in_state(MainState::Game)),
        )
        .add_systems(
            Update,
            (
                pause::toggle.run_if(input_just_pressed(KeyCode::Escape)),
                player::visual_state,
                update_camera,
                exit_game.run_if(input_just_pressed(KeyCode::F4).or(on_event::<PlayerDeath>)),
                update_stats.run_if(on_event::<ItemPickup>),
                pickup_items,
                hud::update_health,
                hud::update_equipment.run_if(resource_changed::<PlayerEquipment>),
            )
                .run_if(in_state(MainState::Game)),
        );
    // .add_systems(OnExit(GameState::Game), on_game_exit);
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_color(Color::BLACK, Vec2::from((640.0, 360.0))),
        StateScoped(MainState::Game),
    ));
    commands.spawn((
        Item::Banana,
        Sprite::from_image(asset_server.load("banana.png")),
        Transform::from_translation(Vec3::from((100.0, -100.0, 0.4))),
        StateScoped(MainState::Game),
    ));
}

fn exit_game(mut next_state: ResMut<NextState<MainState>>) {
    next_state.set(MainState::Menu);
}

fn spawn_enemy(time: Res<Time<Fixed>>, mut commands: Commands, mut enemy_spawn: ResMut<EnemySpawn>) {
    if enemy_spawn.0.tick(time.delta()).finished() {
        commands.spawn((
            Enemy,
            Health(ENEMY_HEALTH),
            Velocity(Vec3::ZERO),
            Sprite::from_color(Color::srgb(1.0, 0.0, 0.6), Vec2::from((ENEMY_SIZE, ENEMY_SIZE))),
            Transform::from_translation(Vec3::from((320.0, 180.0, 0.5))),
            StateScoped(MainState::Game),
        ));
    }
}

fn enemy_state(
    mut q_enemies: Query<(&GlobalTransform, &mut Velocity), With<Enemy>>,
    q_player: Query<&GlobalTransform, (With<Player>, Without<Enemy>)>,
) -> Result {
    let player_pos = q_player.single()?.translation();
    q_enemies
        .iter_mut()
        .for_each(|(t, mut v)| v.0 = ENEMY_SPEED * (player_pos - t.translation()).normalize_or_zero());

    Ok(())
}

fn despawn_unhealthy(mut commands: Commands, query: Query<(Entity, &Health), Without<Player>>) {
    query.iter().for_each(|(e, h)| {
        if h.0 <= 0.0 {
            commands.entity(e).despawn();
        }
    })
}

// TODO Solve collisions with events / observers

fn enemy_hit(
    mut commands: Commands,
    mut enemies: Query<(&mut Health, &GlobalTransform), With<Enemy>>,
    projectiles: Query<(Entity, &GlobalTransform), (With<Projectile>, Without<Enemy>)>,
) {
    projectiles.iter().for_each(|(projectile, projectile_transform)| {
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
fn update_stats(mut q_stats: Query<&mut Stats>, eq: Res<PlayerEquipment>) -> Result {
    q_stats.single_mut()?.apply_equipment(&eq);
    Ok(())
}

fn pickup_items(
    mut commands: Commands,
    q_player: Query<&GlobalTransform, With<Player>>,
    q_items: Query<(Entity, &Item, &GlobalTransform), Without<Player>>,
    mut equipment: ResMut<PlayerEquipment>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut pickup_events: EventWriter<ItemPickup>,
) -> Result {
    let player_pos = q_player.single()?.translation();
    if keyboard.just_pressed(KeyCode::KeyE) {
        // Finds the closest item within the `INTERACTION_DISTANCE` and picks it up.
        q_items
            .iter()
            .map(|(e, item, pos)| (e, item, player_pos.distance(pos.translation())))
            .filter(|(_, _, distance)| *distance < INTERACTION_DISTANCE)
            .min_by(|(_, _, x), (_, _, y)| x.partial_cmp(y).unwrap_or(Ordering::Equal))
            .map(|(entity, item, _)| {
                equipment.pickup(item.clone());
                commands.entity(entity).despawn();
                pickup_events.write_default();
            });
    }

    Ok(())
}

fn physics(time_fixed: Res<Time<Fixed>>, mut query: Query<(&mut Transform, &Velocity)>) {
    query
        .iter_mut()
        .for_each(|(mut transform, vel)| transform.translation += vel.0 * time_fixed.delta_secs());
}

fn update_camera(
    time: Res<Time>,
    cursor_position: Res<CursorPosition>,
    mut camera_query: Query<(&mut Transform, &GlobalTransform), With<Camera2d>>,
    // the Without<Camera2d> is required because both query Transform
    player_transform: Single<&Transform, (With<Player>, Without<Camera2d>)>,
) -> Result {
    let (mut camera_transform, camera_global_transform) = camera_query.single_mut()?;

    let camera_goal = match cursor_position.0 {
        // in case of no cursor on the screen just follow the player
        None => player_transform.translation,
        Some(cursor_position) => {
            // calculate vector from camera to cursor and add that to player
            let direction = cursor_position.extend(0.0) - camera_global_transform.translation();
            player_transform.translation + CURSOR_CAMERA_INFLUENCE * direction
        }
    }
    .with_z(camera_global_transform.translation().z);

    camera_transform
        .translation
        .smooth_nudge(&camera_goal, CAMERA_SPEED, time.delta_secs());

    Ok(())
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, SubStates)]
#[source(MainState = MainState::Game)]
enum GameSubState {
    #[default]
    Running,
    Paused,
}

#[derive(Event, Default)]
pub struct ItemPickup;
