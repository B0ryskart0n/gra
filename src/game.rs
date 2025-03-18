// TODO Consider merging `Position` and `Tranfsorm`, since it may just not be worth it to run the physics separately

use super::CursorPosition;
use super::GameState;
use super::utils::*;
use bevy::color::palettes::css::{BLUE, GREY, RED};
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 250.0;
/// Actually, rate of exponential decay in the distance between camera and it's goal
const CAMERA_SPEED: f32 = 6.0;
const CURSOR_CAMERA_INFLUENCE: f32 = 0.4;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), enter_game)
        .add_systems(Update, handle_player_input.run_if(in_state(GameState::Game)))
        .add_systems(Update, display.run_if(in_state(GameState::Game)))
        .add_systems(Update, primary.run_if(in_state(GameState::Game)))
        .add_systems(Update, update_camera.run_if(in_state(GameState::Game)))
        .add_systems(Update, exit_game_check.run_if(in_state(GameState::Game)))
        .add_systems(
            FixedUpdate,
            // important to keep the ordering so the movement is smooth
            (player_state, physics).chain().run_if(in_state(GameState::Game)),
        )
        .add_systems(OnExit(GameState::Game), exit_game);
}

/// Accumulates the input on `Update` schedule, is cleared and taken into account in `player_physics`,
/// which runs on `FixedUpdate` schedule.
#[derive(Resource, Default)]
struct PlayerInput {
    direction: Vec3,
    dash: bool,
    primary: bool,
}

#[derive(Component)]
struct Projectile;
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);
#[derive(Component)]
struct Position(Vec3);

/// Represents the internal, underlying state used in the game logic, not on the UI level.
#[derive(Default, Component)]
struct PlayerState {
    // TODO maybe handle dashing / primary with a union since those should be exclusive
    dashing: bool,
    primary: bool,
}

fn enter_game(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.init_resource::<PlayerInput>();
    commands.insert_resource(DashTimer(Timer::from_seconds(0.5, TimerMode::Once)));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1280., 720.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(GREY))),
        StateScoped(GameState::Game),
    ));
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(25.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
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

fn handle_player_input(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_input: ResMut<PlayerInput>,
) {
    let left = keyboard.pressed(KeyCode::KeyA);
    let right = keyboard.pressed(KeyCode::KeyD);
    let down = keyboard.pressed(KeyCode::KeyS);
    let up = keyboard.pressed(KeyCode::KeyW);

    let direction = match (left, right, down, up) {
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
    player_input.direction += direction;

    // TODO Investigate whether this causes problems if Update runs twice with no FixedUpdate
    player_input.dash = keyboard.any_just_pressed(vec![KeyCode::ShiftLeft, KeyCode::Space]);
    player_input.primary = mouse.pressed(MouseButton::Left);
}

#[derive(Resource)]
struct DashTimer(Timer);

// TODO Split into three stages: input handling (UI) -> state (Fixed) -> physics (Fixed) -> UI
fn player_state(
    time_fixed: Res<Time<Fixed>>,
    mut query: Query<(&mut PlayerState, &mut Velocity), With<Player>>,
    mut input: ResMut<PlayerInput>,
    mut dash_timer: ResMut<DashTimer>,
) {
    let (mut state, mut velocity) = query.single_mut();

    let dt = time_fixed.delta();

    // conscious short circuit
    if state.dashing && dash_timer.0.tick(dt).finished() {
        state.dashing = false;
        dash_timer.0.reset();
    }

    if input.dash {
        state.dashing = input.dash
    }

    input.dash = false;

    state.primary = input.primary;

    // double the speed when dashing
    let speed_mult = 1.0 + f32::from(state.dashing);

    velocity.0 = input.direction * speed_mult * PLAYER_SPEED;
    // handled all input that accumulated since last `Update`
    input.direction = Vec3::ZERO;
}

fn physics(time_fixed: Res<Time<Fixed>>, mut query: Query<(&mut Position, &Velocity)>) {
    query
        .iter_mut()
        .for_each(|(mut pos, vel)| pos.0 += vel.0 * time_fixed.delta_secs());
}

fn primary(
    mut commands: Commands,
    query: Query<(&Position, &PlayerState), With<Player>>,
    cursor_position: Res<CursorPosition>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (position, player_state) = query.single();

    // TODO Despawn bullets after some time
    if player_state.primary {
        commands.spawn((
            Projectile,
            Mesh2d(meshes.add(Circle::new(5.))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(BLUE))),
            Transform::from_translation(position.0),
            Position(position.0),
            // TODO add handling when the cursor is not on the screen
            Velocity(cursor_position.0.unwrap().extend(0.0) - position.0),
            StateScoped(GameState::Game),
        ));
    }
}

/// Updates the visible components based on the physical state.
fn display(
    mut player: Query<(&PlayerState, &mut MeshMaterial2d<ColorMaterial>), With<Player>>,
    mut query: Query<(&Position, &mut Transform)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (state, mut material) = player.single_mut();

    // TODO Introduce reusing colors instead of adding new ones.
    if state.dashing {
        material.0 = materials.add(ColorMaterial::from_color(BLUE));
    } else {
        material.0 = materials.add(ColorMaterial::from_color(RED));
    }

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
