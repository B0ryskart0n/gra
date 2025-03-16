use super::CursorPosition;
use super::GameState;
use super::utils::*;
use bevy::color::palettes::css::{BLUE, GREY, RED};
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 200.0;
/// Actually, rate of exponential decay in the distance between camera and it's goal
const CAMERA_SPEED: f32 = 5.0;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), enter_game)
        .add_systems(FixedUpdate, player_physics.run_if(in_state(GameState::Game)))
        .add_systems(Update, handle_player_input.run_if(in_state(GameState::Game)))
        .add_systems(Update, update_player.run_if(in_state(GameState::Game)))
        .add_systems(Update, update_camera.run_if(in_state(GameState::Game)))
        .add_systems(Update, exit_game_check.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), exit_game);
}

/// Accumulates the input on `Update` schedule, is cleared and taken into account in `player_physics`,
/// which runs on `FixedUpdate` schedule.
#[derive(Resource, Default)]
struct PlayerInput {
    direction: Vec3,
    dash: bool,
}

#[derive(Component)]
struct Player;

/// Represents the internal, underlying state used in the game logic, not on the UI level.
#[derive(Component)]
struct PlayerState {
    position: Vec3,
    dashing: bool,
}
impl Default for PlayerState {
    fn default() -> Self {
        PlayerState {
            position: Vec3::new(0.0, 0.0, 1.0),
            dashing: false,
        }
    }
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
        Transform::default(),
        StateScoped(GameState::Game),
    ));
}
fn exit_game(mut commands: Commands) {
    commands.remove_resource::<PlayerInput>();
}

fn handle_player_input(keyboard: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
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

    if keyboard.any_just_pressed(vec![KeyCode::ShiftLeft, KeyCode::Space]) {
        player_input.dash = true
    }

    player_input.direction += direction;
}

#[derive(Resource, Deref, DerefMut)]
struct DashTimer(Timer);

fn player_physics(
    time_fixed: Res<Time<Fixed>>,
    mut query: Query<&mut PlayerState, With<Player>>,
    mut player_input: ResMut<PlayerInput>,
    mut dash_timer: ResMut<DashTimer>,
) {
    let mut state = query.single_mut();

    let dt = time_fixed.delta();

    // conscious short circuit
    if state.dashing && dash_timer.0.tick(dt).finished() {
        state.dashing = false;
        dash_timer.0.reset();
    }

    if player_input.dash {
        state.dashing = player_input.dash
    }

    player_input.dash = false;

    // double the speed when dashing
    let speed_mult = 1.0 + f32::from(state.dashing);

    state.position += player_input.direction * speed_mult * PLAYER_SPEED * dt.as_secs_f32();
    player_input.direction = Vec3::ZERO;
}

fn update_player(
    mut query: Query<(&mut Transform, &PlayerState, &mut MeshMaterial2d<ColorMaterial>), With<Player>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (mut transform, state, mut material) = query.single_mut();

    // TODO Introduce reusing colors instead of adding new ones.
    if state.dashing {
        material.0 = materials.add(ColorMaterial::from_color(BLUE));
    } else {
        material.0 = materials.add(ColorMaterial::from_color(RED));
    }

    transform.translation = state.position;
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
            player_transform.translation + direction.clamp_length_max(PLAYER_SPEED)
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
