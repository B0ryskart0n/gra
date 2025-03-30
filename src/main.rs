use bevy::{dev_tools::fps_overlay::FpsOverlayPlugin, prelude::*};

mod assets;
mod game;
mod menu;
mod settings;
mod splash;
mod utils;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    //Settings,
    Game,
    //Pause
}

/// Cursor world position.
#[derive(Resource, Default)]
struct CursorPosition(Option<Vec2>);

fn main() {
    App::new()
        .enable_state_scoped_entities::<GameState>()
        .init_state::<GameState>() // Initial state will be the #[default]
        .init_resource::<CursorPosition>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, update_cursor_position)
        .add_plugins((DefaultPlugins, FpsOverlayPlugin::default()))
        .add_plugins(splash::splash_plugin)
        .add_plugins(menu::menu_plugin)
        .add_plugins(settings::plugin)
        .add_plugins(game::game_plugin)
        .run();
}

fn update_cursor_position(
    mut cursor_position: ResMut<CursorPosition>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    cursor_position.0 = window
        .cursor_position()
        .map(|viewport_position| camera.viewport_to_world_2d(camera_transform, viewport_position))
        .map(|res| res.ok())
        .flatten(); // from Option<Option<Vec2> to Option<Vec2>
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
