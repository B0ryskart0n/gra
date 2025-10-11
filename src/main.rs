use bevy::prelude::*;

mod assets;
mod game;
mod menu;
mod settings;
mod splash;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // .add_plugins(bevy::dev_tools::fps_overlay::FpsOverlayPlugin::default())
        // Needs to be done after StatesPlugin (part of DefaultPlugins)
        .init_state::<MainState>() // Initial state will be the #[default]
        .init_resource::<CursorPosition>()
        .add_systems(Startup, spawn_camera)
        .add_systems(
            RunFixedMainLoop,
            update_cursor_position.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
        )
        .add_plugins(splash::splash_plugin)
        .add_plugins(menu::menu_plugin)
        .add_plugins(settings::plugin)
        .add_plugins(game::game_plugin)
        .run();
}

fn update_cursor_position(
    mut cursor_position: ResMut<CursorPosition>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) -> Result {
    let (camera, camera_transform) = q_camera.single()?;
    let window = q_window.single()?;

    cursor_position.0 = window
        .cursor_position()
        .map(|viewport_position| camera.viewport_to_world_2d(camera_transform, viewport_position))
        .and_then(|res| res.ok());
    Ok(())
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(PrimaryCamera);
}

#[derive(Component)]
#[require(Camera2d)]
struct PrimaryCamera;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
#[states(scoped_entities)]
enum MainState {
    #[default]
    Splash,
    Menu,
    Settings,
    Game,
}

/// Cursor world position.
#[derive(Resource, Default)]
struct CursorPosition(Option<Vec2>);
