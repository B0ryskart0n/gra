use bevy::prelude::*;
use bevy::window::WindowResolution;

mod assets;
mod game;
mod menu;
mod settings;
mod splash;
mod utils;

use settings::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        decorations: false,
                        resolution: WindowResolution::new(LOGICAL_WIDTH as f32 * SCALE, LOGICAL_HEIGHT as f32 * SCALE)
                            .with_scale_factor_override(SCALE),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // Needs to be done after DefaultPlugins, because DefaultPlugins initializes StateTransitions
        .init_state::<GameState>() // Initial state will be the #[default]
        .enable_state_scoped_entities::<GameState>()
        .init_resource::<CursorPosition>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, update_cursor_position)
        .add_plugins(splash::splash_plugin)
        .add_plugins(menu::menu_plugin)
        .add_plugins(settings::plugin)
        .add_plugins(game::game_plugin)
        // .add_plugins(bevy::dev_tools::fps_overlay::FpsOverlayPlugin::default())
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
    commands.spawn((Camera2d, PrimaryCamera));
}

#[derive(Component)]
struct PrimaryCamera;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Settings,
    Game,
}

/// Cursor world position.
#[derive(Resource, Default)]
struct CursorPosition(Option<Vec2>);
