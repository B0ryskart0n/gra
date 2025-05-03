mod components;
mod events;
mod hud;
mod resources;

use bevy::prelude::*;
use std::cmp::Ordering;
use std::time::Duration;

use crate::CursorPosition;
use crate::GameState;
use crate::utils::*;
use components::*;
use events::*;
use resources::*;

// TODO Shouldn't all sizes be whole number?
const INTERACTION_DISTANCE: f32 = 30.0;
const ENEMY_SIZE: f32 = 15.0;
const ENEMY_HEALTH: f32 = 3.0;
const ENEMY_SPEED: f32 = 100.0;
const PROJECTILE_SIZE: f32 = 2.0;
const PROJECTILE_SPEED: f32 = 400.0;
const PROJECTILE_LIFETIME: f32 = 1.0;
const PLAYER_SIZE: f32 = 25.0;
/// Rate of exponential decay in the distance between camera and it's goal.
const CAMERA_SPEED: f32 = 8.0;
const CURSOR_CAMERA_INFLUENCE: f32 = 0.3;

pub fn game_plugin(app: &mut App) {
    // Since I want to rely on `resource_changed` condition I need to initiate
    // those resources at the top level instead of `OnEnter(GameState::Game)`.
    app.init_resource::<PlayerInput>()
        .init_resource::<DashTimer>()
        .init_resource::<AttackTimer>()
        .init_resource::<EnemySpawn>()
        .init_resource::<PlayerEquipment>()
        .add_state_scoped_event::<PlayerDeath>(GameState::Game)
        .add_state_scoped_event::<ItemPickup>(GameState::Game)
        .add_systems(OnEnter(GameState::Game), (reset_camera, spawn, hud::spawn))
        .add_systems(
            RunFixedMainLoop,
            handle_player_input
                .run_if(in_state(GameState::Game))
                .in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
        )
        .add_systems(
            FixedUpdate,
            (
                spawn_enemy,
                player_hit,
                (enemy_hit, despawn_unhealthy).chain(),
                attack,
                lifetime,
                (player_state, enemy_state, physics).chain(),
            )
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (
                display_player_state,
                update_camera,
                check_game_exit,
                update_stats.run_if(on_event::<ItemPickup>),
                pickup_items,
                hud::update_health,
                hud::update_equipment.run_if(resource_changed::<PlayerEquipment>),
            )
                .run_if(in_state(GameState::Game)),
        );
    // .add_systems(OnExit(GameState::Game), on_game_exit);
}

fn lifetime(time: Res<Time>, mut commands: Commands, mut query: Query<(Entity, &mut Lifetime)>) {
    let dt = time.delta();
    query.iter_mut().for_each(|(e, mut l)| {
        if l.0.tick(dt).finished() {
            commands.entity(e).despawn()
        }
    })
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_color(Color::BLACK, Vec2::from((640.0, 360.0))),
        StateScoped(GameState::Game),
    ));
    commands.spawn((
        Player,
        Health(PLAYER_HEALTH),
        Stats::default(),
        Sprite::from_color(Color::WHITE, Vec2::from((PLAYER_SIZE, PLAYER_SIZE))),
        PlayerState::default(),
        Transform::from_translation(Vec3::from((0.0, 0.0, 1.0))),
        Velocity(Vec3::ZERO),
        StateScoped(GameState::Game),
    ));
    commands.spawn((
        Item::Banana,
        Sprite::from_image(asset_server.load("banana.png")),
        Transform::from_translation(Vec3::from((100.0, -100.0, 0.4))),
        StateScoped(GameState::Game),
    ));
}

fn check_game_exit(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    death_events: EventReader<PlayerDeath>,
) {
    if keyboard.just_pressed(KeyCode::Escape) || !death_events.is_empty() {
        next_state.set(GameState::Menu);
    }
}

fn spawn_enemy(time: Res<Time<Fixed>>, mut commands: Commands, mut enemy_spawn: ResMut<EnemySpawn>) {
    if enemy_spawn.0.tick(time.delta()).finished() {
        commands.spawn((
            Enemy,
            Health(ENEMY_HEALTH),
            Velocity(Vec3::ZERO),
            Sprite::from_color(Color::srgb(1.0, 0.0, 0.6), Vec2::from((ENEMY_SIZE, ENEMY_SIZE))),
            Transform::from_translation(Vec3::from((320.0, 180.0, 0.5))),
            StateScoped(GameState::Game),
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

fn hit_enemy_projectile(enemy_pos: Vec3, projectile_pos: Vec3) -> bool {
    return enemy_pos.x - ENEMY_SIZE / 2.0 < projectile_pos.x
        && projectile_pos.x < enemy_pos.x + ENEMY_SIZE / 2.0
        && enemy_pos.y - ENEMY_SIZE / 2.0 < projectile_pos.y
        && projectile_pos.y < enemy_pos.y + ENEMY_SIZE / 2.0;
}
fn hit_player_enemy(player_pos: Vec3, enemy_pos: Vec3) -> bool {
    return player_pos.x - PLAYER_SIZE / 2.0 < enemy_pos.x + ENEMY_SIZE / 2.0
        && player_pos.x + PLAYER_SIZE / 2.0 > enemy_pos.x - ENEMY_SIZE / 2.0
        && player_pos.y - PLAYER_SIZE / 2.0 < enemy_pos.y + ENEMY_SIZE / 2.0
        && player_pos.y + PLAYER_SIZE / 2.0 > enemy_pos.y - ENEMY_SIZE / 2.0;
}

// TODO Solve collisions with events / observers

fn enemy_hit(
    mut commands: Commands,
    mut enemies: Query<(&mut Health, &GlobalTransform), With<Enemy>>,
    projectiles: Query<(Entity, &GlobalTransform), (With<Projectile>, Without<Enemy>)>,
) {
    projectiles.iter().for_each(|(projectile, projectile_transform)| {
        for (mut health, enemy_position) in enemies.iter_mut() {
            if hit_enemy_projectile(enemy_position.translation(), projectile_transform.translation()) {
                health.0 = health.0 - 1.0;
                commands.entity(projectile).despawn();
                // Projectile despawned, so can't influence other enemies. Go to next projectile.
                // Maybe if in the future projectiles can pass through then handle differently.
                break;
            }
        }
    })
}
fn player_hit(
    q_enemies: Query<&GlobalTransform, With<Enemy>>,
    mut q_player: Query<(&mut Health, &GlobalTransform, &PlayerState), (With<Player>, Without<Enemy>)>,
    mut death_events: EventWriter<PlayerDeath>,
) -> Result {
    let (mut player_health, player_transform, player_state) = q_player.single_mut()?;
    let damage = match *player_state {
        PlayerState::Dashing(_) => 0.0,
        _ => q_enemies
            .iter()
            .map(|enemy_transform| hit_player_enemy(player_transform.translation(), enemy_transform.translation()))
            .filter(|hit| *hit)
            .count() as f32,
    };

    player_health.0 -= damage;
    if player_health.0 <= 0.0 {
        death_events.write_default();
    }

    Ok(())
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

/// In case of high frame rate (bigger than `FixedTime` 64Hz), if one swift button press is registered and
/// that input is overriden in  next schedule run (when the button is already released) and
/// the `FixedUpdate` schedule did not run, because the two frames were too close to each other,
/// then the swift input is effectively discarded.
fn handle_player_input(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_input: ResMut<PlayerInput>,
) {
    let left = keyboard.pressed(KeyCode::KeyA);
    let right = keyboard.pressed(KeyCode::KeyD);
    let down = keyboard.pressed(KeyCode::KeyS);
    let up = keyboard.pressed(KeyCode::KeyW);

    player_input.direction = match (left, right, down, up) {
        (false, false, false, false)
        | (true, true, true, true)
        | (true, true, false, false)
        | (false, false, true, true) => Vec3::ZERO,
        (false, true, false, false) | (false, true, true, true) => DIRECTION_RIGHT,
        (false, true, false, true) => DIRECTION_UPRIGHT,
        (false, false, false, true) | (true, true, false, true) => DIRECTION_UP,
        (true, false, false, true) => DIRECTION_UPLEFT,
        (true, false, false, false) | (true, false, true, true) => DIRECTION_LEFT,
        (true, false, true, false) => DIRECTION_DOWNLEFT,
        (false, false, true, false) | (true, true, true, false) => DIRECTION_DOWN,
        (false, true, true, false) => DIRECTION_DOWNRIGHT,
    };

    player_input.dash = keyboard.any_just_pressed(vec![KeyCode::ShiftLeft, KeyCode::Space]);
    player_input.attack = mouse.pressed(MouseButton::Left);
}

fn player_state(
    time_fixed: Res<Time<Fixed>>,
    mut query: Query<(&mut PlayerState, &mut Velocity, &Stats), With<Player>>,
    input: Res<PlayerInput>,
    mut dash_timer: ResMut<DashTimer>,
) -> Result {
    let (mut state, mut velocity, stats) = query.single_mut()?;

    let dt = time_fixed.delta();

    if state.is_dashing() && dash_timer.0.tick(dt).finished() {
        *state = PlayerState::Idle;
        dash_timer.0.reset();
    }
    if !state.is_dashing() {
        *state = match (input.dash, input.attack) {
            (true, _) => PlayerState::Dashing(input.direction),
            (false, true) => PlayerState::Attacking,
            (false, false) => PlayerState::Idle,
        };
    }

    let base_velocity = match *state {
        PlayerState::Idle => 1.0 * input.direction,
        PlayerState::Dashing(dash) => 2.5 * dash,
        PlayerState::Attacking => 0.5 * input.direction,
    };

    velocity.0 = base_velocity * stats.movement_speed;

    Ok(())
}

fn physics(time_fixed: Res<Time<Fixed>>, mut query: Query<(&mut Transform, &Velocity)>) {
    query
        .iter_mut()
        .for_each(|(mut transform, vel)| transform.translation += vel.0 * time_fixed.delta_secs());
}

fn attack(
    time_fixed: Res<Time<Fixed>>,
    mut commands: Commands,
    query: Query<(&GlobalTransform, &PlayerState, &Stats), With<Player>>,
    cursor_position: Res<CursorPosition>,
    mut attack_timer: ResMut<AttackTimer>,
) -> Result {
    let (player_transform, player_state, stats) = query.single()?;
    let player_position = player_transform.translation();
    let dt = time_fixed.delta_secs();

    attack_timer.0.tick(Duration::from_secs_f32(dt * stats.attack_speed));

    if *player_state == PlayerState::Attacking && attack_timer.0.finished() {
        commands.spawn((
            Projectile,
            Sprite::from_color(Color::WHITE, Vec2::from((PROJECTILE_SIZE, PROJECTILE_SIZE))),
            Transform::from_translation(player_position),
            Velocity(Vec3::lerp(
                Vec3::ZERO,
                (cursor_position.0.unwrap_or(Vec2::X).extend(0.0) - player_position).normalize(),
                PROJECTILE_SPEED,
            )),
            StateScoped(GameState::Game),
            Lifetime(Timer::from_seconds(PROJECTILE_LIFETIME, TimerMode::Once)),
        ));
        attack_timer.0.reset();
    }

    Ok(())
}

fn display_player_state(mut query: Query<(&mut Sprite, &PlayerState), Changed<PlayerState>>) {
    query.iter_mut().for_each(|(mut sprite, state)| match *state {
        PlayerState::Idle => sprite.color = Color::srgb(0.1, 1.0, 0.1),
        PlayerState::Attacking => sprite.color = Color::srgb(1.0, 0.1, 0.1),
        PlayerState::Dashing(_) => sprite.color = Color::srgb(0.1, 0.1, 1.0),
    })
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
