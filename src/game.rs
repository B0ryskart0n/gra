use super::CursorPosition;
use super::GameState;
use super::utils::*;
use bevy::prelude::*;

const ENEMY_SIZE: f32 = 30.0;
const ENEMY_HEALTH: f32 = 3.0;
const ENEMY_SPAWN_RATE: f32 = 5.0;
const ENEMY_SPEED: f32 = 150.0;
const PROJECTILE_SPEED: f32 = 750.0;
const PROJECTILE_LIFETIME: f32 = 1.0;
const PLAYER_SIZE: f32 = 50.0;
const PLAYER_SPEED: f32 = 250.0;
const PLAYER_HEALTH: f32 = 5.0;
const ATTACK_SPEED: f32 = 2.0;
/// Actually, rate of exponential decay in the distance between camera and it's goal
const CAMERA_SPEED: f32 = 6.0;
const CURSOR_CAMERA_INFLUENCE: f32 = 0.4;

pub fn game_plugin(app: &mut App) {
    app.add_state_scoped_event::<PlayerDeath>(GameState::Game)
        .add_systems(OnEnter(GameState::Game), enter_game)
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
            (display_player_state, update_camera, exit_game_check).run_if(in_state(GameState::Game)),
        )
        .add_systems(OnExit(GameState::Game), exit_game);
}

#[derive(Event, Default)]
struct PlayerDeath;

#[derive(Resource, Default)]
struct PlayerInput {
    direction: Vec3,
    dash: bool,
    attack: bool,
}
#[derive(Resource)]
struct AttackSpeed(Timer);
#[derive(Resource)]
struct DashTimer(Timer);
#[derive(Resource)]
struct EnemySpawn(Timer);

#[derive(Component)]
struct Projectile;
#[derive(Component)]
struct Player;
#[derive(PartialEq, Eq, Default, Component)]
enum PlayerState {
    #[default]
    Idle,
    Dashing,
    Attacking,
}
#[derive(Component)]
struct Enemy;
#[derive(Component)]
struct Health(f32);
#[derive(Component)]
struct Velocity(Vec3);
#[derive(Component)]
struct Lifetime(Timer);
fn lifetime(time: Res<Time>, mut commands: Commands, mut query: Query<(Entity, &mut Lifetime)>) {
    let dt = time.delta();
    query.iter_mut().for_each(|(e, mut l)| {
        if l.0.tick(dt).finished() {
            commands.entity(e).despawn_recursive()
        }
    })
}

fn enter_game(mut commands: Commands) {
    commands.init_resource::<PlayerInput>();
    commands.insert_resource(DashTimer(Timer::from_seconds(0.5, TimerMode::Once)));
    commands.insert_resource(AttackSpeed(Timer::from_seconds(1.0 / ATTACK_SPEED, TimerMode::Once)));
    commands.insert_resource(EnemySpawn(Timer::from_seconds(ENEMY_SPAWN_RATE, TimerMode::Repeating)));

    commands.spawn((
        Sprite::from_color(Color::BLACK, Vec2::from((1280.0, 720.0))),
        StateScoped(GameState::Game),
    ));
    commands.spawn((
        Player,
        Health(PLAYER_HEALTH),
        Sprite::from_color(Color::WHITE, Vec2::from((PLAYER_SIZE, PLAYER_SIZE))),
        PlayerState::default(),
        Transform::from_translation(Vec3::from((0.0, 0.0, 1.0))),
        Velocity(Vec3::ZERO),
        StateScoped(GameState::Game),
    ));
}
fn exit_game(mut commands: Commands) {
    commands.remove_resource::<PlayerInput>();
    commands.remove_resource::<DashTimer>();
    commands.remove_resource::<AttackSpeed>();
}

fn spawn_enemy(time: Res<Time>, mut commands: Commands, mut enemy_spawn: ResMut<EnemySpawn>) {
    if enemy_spawn.0.tick(time.delta()).finished() {
        commands.spawn((
            Enemy,
            Health(ENEMY_HEALTH),
            Velocity(Vec3::ZERO),
            Sprite::from_color(Color::srgb(1.0, 0.0, 0.6), Vec2::from((ENEMY_SIZE, ENEMY_SIZE))),
            Transform::from_translation(Vec3::from((300.0, 300.0, 0.5))),
            StateScoped(GameState::Game),
        ));
    }
}

fn enemy_state(
    mut q_enemies: Query<(&GlobalTransform, &mut Velocity), With<Enemy>>,
    q_player: Query<&GlobalTransform, (With<Player>, Without<Enemy>)>,
) {
    let player_pos = q_player.single().translation();
    q_enemies
        .iter_mut()
        .for_each(|(t, mut v)| v.0 = ENEMY_SPEED * (player_pos - t.translation()).normalize_or_zero());
}

fn despawn_unhealthy(mut commands: Commands, query: Query<(Entity, &Health), Without<Player>>) {
    query.iter().for_each(|(e, h)| {
        if h.0 <= 0.0 {
            commands.entity(e).despawn_recursive();
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

// TODO Maybe solve collisions with events

fn enemy_hit(
    mut commands: Commands,
    mut enemies: Query<(&mut Health, &GlobalTransform), With<Enemy>>,
    projectiles: Query<(Entity, &GlobalTransform), (With<Projectile>, Without<Enemy>)>,
) {
    projectiles.iter().for_each(|(projectile, projectile_transform)| {
        for (mut health, enemy_position) in enemies.iter_mut() {
            if hit_enemy_projectile(enemy_position.translation(), projectile_transform.translation()) {
                health.0 = health.0 - 1.0;
                commands.entity(projectile).despawn_recursive();
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
) {
    let (mut player_health, player_transform, player_state) = q_player.single_mut();
    let damage = match *player_state {
        PlayerState::Dashing => 0.0,
        _ => q_enemies
            .iter()
            .map(|enemy_transform| hit_player_enemy(player_transform.translation(), enemy_transform.translation()))
            .map(|b| b as i32 as f32)
            .sum(),
    };

    player_health.0 -= damage;
    if player_health.0 <= 0.0 {
        death_events.send_default();
    }
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
    mut query: Query<(&mut PlayerState, &mut Velocity), With<Player>>,
    input: Res<PlayerInput>,
    mut dash_timer: ResMut<DashTimer>,
) {
    let (mut state, mut velocity) = query.single_mut();

    let dt = time_fixed.delta();

    if *state == PlayerState::Dashing && dash_timer.0.tick(dt).finished() {
        *state = PlayerState::Idle;
        dash_timer.0.reset();
    }
    if *state != PlayerState::Dashing {
        *state = match (input.dash, input.attack) {
            (true, _) => PlayerState::Dashing,
            (false, true) => PlayerState::Attacking,
            (false, false) => PlayerState::Idle,
        };
    }

    let speed_mult = match *state {
        PlayerState::Idle => 1.0,
        PlayerState::Dashing => 2.5,
        PlayerState::Attacking => 0.75,
    };

    velocity.0 = input.direction * speed_mult * PLAYER_SPEED;
}

fn physics(time_fixed: Res<Time<Fixed>>, mut query: Query<(&mut Transform, &Velocity)>) {
    query
        .iter_mut()
        .for_each(|(mut transform, vel)| transform.translation += vel.0 * time_fixed.delta_secs());
}

fn attack(
    time_fixed: Res<Time<Fixed>>,
    mut commands: Commands,
    query: Query<(&GlobalTransform, &PlayerState), With<Player>>,
    cursor_position: Res<CursorPosition>,
    mut attack_speed: ResMut<AttackSpeed>,
) {
    let (player_transform, player_state) = query.single();
    let player_position = player_transform.translation();

    attack_speed.0.tick(time_fixed.delta());

    if *player_state == PlayerState::Attacking && attack_speed.0.finished() {
        commands.spawn((
            Projectile,
            Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::from((5.0, 5.0))),
            Transform::from_translation(player_position),
            Velocity(Vec3::lerp(
                Vec3::ZERO,
                (cursor_position.0.unwrap_or(Vec2::X).extend(0.0) - player_position).normalize(),
                PROJECTILE_SPEED,
            )),
            StateScoped(GameState::Game),
            Lifetime(Timer::from_seconds(PROJECTILE_LIFETIME, TimerMode::Once)),
        ));
        attack_speed.0.reset();
    }
}

fn display_player_state(mut query: Query<(&mut Sprite, &PlayerState), Changed<PlayerState>>) {
    query.iter_mut().for_each(|(mut sprite, state)| match *state {
        PlayerState::Idle => sprite.color = Color::srgb(0.1, 1.0, 0.1),
        PlayerState::Attacking => sprite.color = Color::srgb(1.0, 0.1, 0.1),
        PlayerState::Dashing => sprite.color = Color::srgb(0.0, 0.1, 1.0),
    })
}

fn update_camera(
    time: Res<Time>,
    cursor_position: Res<CursorPosition>,
    mut camera_query: Query<(&mut Transform, &GlobalTransform), With<Camera2d>>,
    // the Without<Camera2d> is required because both query Transform
    player_transform: Single<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let (mut camera_transform, camera_global_transform) = camera_query.single_mut();

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
}

fn exit_game_check(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    death_events: EventReader<PlayerDeath>,
) {
    if keyboard.pressed(KeyCode::Escape) || !death_events.is_empty() {
        next_state.set(GameState::Menu);
    }
}
