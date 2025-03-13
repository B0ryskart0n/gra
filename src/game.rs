use super::CursorPosition;
use super::GameState;
use super::utils::*;
use bevy::color::palettes::css::{GREY, RED};
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 100.0;
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
#[derive(Resource)]
struct PlayerInput(Vec3);

#[derive(Component)]
struct Player;

/// Represents the internal, underlying translation (position) used in the game logic,
/// not on the UI level where `Transform` should be used.
#[derive(Component, Deref, DerefMut)]
struct InternalTranslation(Vec3);

fn enter_game(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(PlayerInput(Vec3::ZERO));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1280., 720.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(GREY))),
        StateScoped(GameState::Game),
    ));
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(25.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        InternalTranslation(Vec3::new(0.0, 0.0, 1.0)),
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

    player_input.0 += direction;
}

fn player_physics(
    time_fixed: Res<Time<Fixed>>,
    mut query: Query<&mut InternalTranslation, With<Player>>,
    mut player_input: ResMut<PlayerInput>,
) {
    let mut position = query.single_mut();
    position.0 += player_input.0 * PLAYER_SPEED * time_fixed.delta_secs();
    player_input.0 = Vec3::ZERO;
}

fn update_player(mut query: Query<(&mut Transform, &InternalTranslation), With<Player>>) {
    let (mut transform, position) = query.single_mut();
    transform.translation = position.0;
}

fn update_camera(
    time: Res<Time>,
    cursor_position: Res<CursorPosition>,
    mut camera_query: Query<(&mut Transform, &GlobalTransform), With<Camera2d>>,
    // the Without<Camera2d> is required because both query Transform
    player_transform: Single<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let (mut camera_transform, camera_global_transform) = camera_query.single_mut();

    // TODO
    // The cursor cannot be moved at the same time as movement keys are pressed, which is weird.
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
