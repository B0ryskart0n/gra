use super::CursorPosition;
use super::GameState;
use super::assets::*;
use super::utils::*;
use bevy::prelude::*;

const PROJECTILE_SPEED: f32 = 1000.0;
const PROJECTILE_LIFETIME: f32 = 1.0;
const PLAYER_SPEED: f32 = 250.0;
const ATTACK_SPEED: f32 = 2.0;
/// Actually, rate of exponential decay in the distance between camera and it's goal
const CAMERA_SPEED: f32 = 6.0;
const CURSOR_CAMERA_INFLUENCE: f32 = 0.4;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), (init_basic_colors, enter_game).chain())
        .add_systems(
            RunFixedMainLoop,
            handle_player_input
                .run_if(in_state(GameState::Game))
                .in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
        )
        .add_systems(
            FixedUpdate,
            (attack, lifetime, (player_state, physics).chain()).run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (display, update_camera, exit_game_check).run_if(in_state(GameState::Game)),
        )
        .add_systems(OnExit(GameState::Game), exit_game);
}

/// Accumulates the input on `Update` schedule, is cleared and taken into account in `FixedUpdate`.
#[derive(Resource, Default)]
struct PlayerInput {
    direction: Vec3,
    dash: bool,
    attack: bool,
}

#[derive(Resource)]
struct AttackSpeed(Timer);

#[derive(Component)]
struct Projectile;
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);
#[derive(Component)]
struct Position(Vec3);

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

/// Represents the internal, underlying state used in the game logic, not on the UI level.
#[derive(PartialEq, Eq, Default, Component)]
enum PlayerState {
    #[default]
    Idle,
    Dashing,
    Attacking,
}

fn init_basic_colors(mut commands: Commands, materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(BasicColorHandles::init_simple_colors(materials));
}

fn enter_game(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, colors: Res<BasicColorHandles>) {
    commands.init_resource::<PlayerInput>();
    commands.insert_resource(DashTimer(Timer::from_seconds(0.5, TimerMode::Once)));
    commands.insert_resource(AttackSpeed(Timer::from_seconds(1.0 / ATTACK_SPEED, TimerMode::Once)));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1280., 720.))),
        MeshMaterial2d(colors.grey.clone()),
        StateScoped(GameState::Game),
    ));
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(25.))),
        MeshMaterial2d(colors.red.clone()),
        PlayerState::default(),
        Position(Vec3::from((0.0, 0.0, 1.0))),
        Velocity(Vec3::ZERO),
        Transform::default(),
        StateScoped(GameState::Game),
    ));
}
fn exit_game(mut commands: Commands) {
    commands.remove_resource::<PlayerInput>();
}

// TODO Think about handling a situation where one swift button press is registered and that input is overriden in
// next schedule run (when the button is already released) and the physics did not run, because the two frames were too
// close to each other. Then the swift input is discarded in the physics simulation.
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

#[derive(Resource)]
struct DashTimer(Timer);

fn player_state(
    time_fixed: Res<Time<Fixed>>,
    mut query: Query<(&mut PlayerState, &mut Velocity), With<Player>>,
    mut input: ResMut<PlayerInput>,
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
    // TODO What happens when there are several FixedUpdates to be run?
    input.direction = Vec3::ZERO;
}

fn physics(time_fixed: Res<Time<Fixed>>, mut query: Query<(&mut Position, &Velocity)>) {
    query
        .iter_mut()
        .for_each(|(mut pos, vel)| pos.0 += vel.0 * time_fixed.delta_secs());
}

fn attack(
    time_fixed: Res<Time<Fixed>>,
    mut commands: Commands,
    query: Query<(&Position, &PlayerState), With<Player>>,
    cursor_position: Res<CursorPosition>,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<BasicColorHandles>,
    mut attack_speed: ResMut<AttackSpeed>,
) {
    let (position, player_state) = query.single();

    attack_speed.0.tick(time_fixed.delta());

    // TODO Despawn bullets after some time
    if *player_state == PlayerState::Attacking && attack_speed.0.finished() {
        commands.spawn((
            Projectile,
            Mesh2d(meshes.add(Circle::new(5.))),
            MeshMaterial2d(colors.green.clone()),
            // TODO Bind Transform and Position together so those cannot be inserted with different values.
            Transform::from_translation(position.0),
            Position(position.0),
            Velocity(Vec3::lerp(
                Vec3::ZERO,
                (cursor_position.0.unwrap_or(Vec2::X).extend(0.0) - position.0).normalize(),
                PROJECTILE_SPEED,
            )),
            StateScoped(GameState::Game),
            Lifetime(Timer::from_seconds(PROJECTILE_LIFETIME, TimerMode::Once)),
        ));

        attack_speed.0.reset();
    }
}

/// Updates the visible components based on the physical state.
fn display(
    mut player: Query<(&PlayerState, &mut MeshMaterial2d<ColorMaterial>), With<Player>>,
    mut query: Query<(&Position, &mut Transform)>,
    colors: Res<BasicColorHandles>,
) {
    let (state, mut material) = player.single_mut();

    material.0 = match *state {
        PlayerState::Dashing => colors.blue.clone(),
        _ => colors.red.clone(),
    };

    query
        .iter_mut()
        .for_each(|(pos, mut transform)| transform.translation = pos.0);
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

fn exit_game_check(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}
