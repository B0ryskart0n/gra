use super::GameState;
use super::utils::*;
use bevy::color::palettes::css::{GREY, RED};
use bevy::prelude::*;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), game_enter)
        // TODO FixedUpdate is good for physics, but updating the Transform (visual component) should be done in Update
        .add_systems(FixedUpdate, game_fixed_update.run_if(in_state(GameState::Game)))
        .add_systems(Update, game_update.run_if(in_state(GameState::Game)));
}

#[derive(Component)]
struct Soldier;

fn game_enter(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000., 700.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(GREY))),
        StateScoped(GameState::Game),
    ));
    commands.spawn((
        Soldier,
        Mesh2d(meshes.add(Circle::new(25.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        Transform::from_xyz(0., 0., 1.),
        StateScoped(GameState::Game),
    ));
}

fn game_fixed_update(
    time_fixed: Res<Time<Fixed>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut soldier: Query<&mut Transform, With<Soldier>>,
) {
    let mut transform = soldier.single_mut();

    let left = keyboard.pressed(KeyCode::KeyA);
    let right = keyboard.pressed(KeyCode::KeyD);
    let down = keyboard.pressed(KeyCode::KeyS);
    let up = keyboard.pressed(KeyCode::KeyW);

    let direction = match (left, right, down, up) {
        (false, false, false, false)
        | (true, true, true, true)
        | (true, true, false, false)
        | (false, false, true, true) => Vec2::ZERO,
        (false, true, false, false) | (false, true, true, true) => DIRECTION_RIGHT,
        (false, true, false, true) => DIRECTION_UPRIGHT,
        (false, false, false, true) | (true, true, false, true) => DIRECTION_UP,
        (true, false, false, true) => DIRECTION_UPLEFT,
        (true, false, false, false) | (true, false, true, true) => DIRECTION_LEFT,
        (true, false, true, false) => DIRECTION_DOWNLEFT,
        (false, false, true, false) | (true, true, true, false) => DIRECTION_DOWN,
        (false, true, true, false) => DIRECTION_DOWNRIGHT,
    };

    let dt = time_fixed.delta_secs();
    // TODO extract speed
    transform.translation += Vec3::from((direction * 50.0, 0.0)) * dt;
}

fn game_update(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Soldier>)>,
    soldier_query: Query<&Transform, (With<Soldier>, Without<Camera2d>)>,
) {
    let mut camera = camera_query.single_mut();
    let soldier = soldier_query.single();

    let Vec3 { x, y, .. } = soldier.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());

    if keyboard.pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}
