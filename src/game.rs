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
        .add_systems(Update, player_update.run_if(in_state(GameState::Game)))
        .add_systems(Update, game_update.run_if(in_state(GameState::Game)))
        .add_systems(Update, exit_game_check.run_if(in_state(GameState::Game)));
}

#[derive(Component)]
struct Player;

/// Represents the internal, underlying translation (position) used in the game logic,
/// not on the UI level where `Transform` should be used.
#[derive(Component, Deref, DerefMut)]
struct InternalTranslation(Vec3);

// TODO Could this be a `Resource`?
/// Accumulates the input on `Update` schedule, is cleared and taken into account in `player_physics`,
/// which runs on `FixedUpdate` schedule.
#[derive(Component)]
struct PlayerInput(Vec3);

fn enter_game(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1280., 720.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(GREY))),
        StateScoped(GameState::Game),
    ));
    commands.spawn((
        Player,
        PlayerInput(Vec3::ZERO),
        Mesh2d(meshes.add(Circle::new(25.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        InternalTranslation(Vec3::new(0.0, 0.0, 1.0)),
        Transform::default(),
        StateScoped(GameState::Game),
    ));
}

fn handle_player_input(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut PlayerInput>) {
    let mut player_input = query.single_mut();

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
    mut query: Query<(&mut InternalTranslation, &mut PlayerInput), With<Player>>,
) {
    let (mut position, mut input) = query.single_mut();
    position.0 += input.0 * PLAYER_SPEED * time_fixed.delta_secs();
    input.0 = Vec3::ZERO;
}

fn player_update(mut query: Query<(&mut Transform, &InternalTranslation), With<Player>>) {
    let (mut transform, position) = query.single_mut();
    transform.translation = position.0;
}

fn game_update(
    time: Res<Time>,
    window_query: Query<&Window>,
    // TODO something tells me that modifying Transform and reading GlobalTransform in the same schedule might be wrong
    mut camera_query: Query<(&Camera, &mut Transform, &GlobalTransform), With<Camera2d>>,
    // the Without<Camera2d> is required because both query Transform
    player_transform: Single<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let (camera, mut camera_transform, camera_global_transform) = camera_query.single_mut();
    let window = window_query.single();

    // TODO
    // The cursor cannot be moved at the same time as movement keys are pressed, which is weird.
    // Also this seems a little skechy with calculating the vector from camera to cursor and then updating
    // camera based on that because the cursor always moves with the camera.
    let camera_goal = match window.cursor_position() {
        // in case of no cursor on the screen just follow the player
        None => player_transform.translation,
        Some(viewport_position) => {
            let cursor_world_position = Vec3::from((
                camera
                    .viewport_to_world_2d(camera_global_transform, viewport_position)
                    .unwrap(),
                0.0,
            ));
            // calculate vector from camera to cursor and add that to player
            let direction = cursor_world_position - camera_global_transform.translation();
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
