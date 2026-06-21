mod assets;
mod game;
mod menu;
mod settings;
mod splash;
mod utils;

use bevy::prelude::*;

// Important for perspective and sprite scaling
const PIXELS_PER_METER: f32 = 16.0;
const METERS_PER_PIXEL: f32 = 1.0 / PIXELS_PER_METER;

// TODO Consider using Events instead of using Messages everywhere.
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        //.insert_resource(UiDebugOptions { enabled: true, ..default() }) // Draws debug borders of Nodes, requires `bevy_ui_debug` feature
        //.add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin::default())
        //.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
        // Needs to be done after StatesPlugin (part of DefaultPlugins)
        .init_state::<MainState>() // Initial state will be the #[default]
        .add_systems(Startup, startup)
        .add_plugins(splash::plugin)
        .add_plugins(menu::plugin)
        .add_plugins(settings::plugin)
        .add_plugins(game::game_plugin)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: METERS_PER_PIXEL,
            ..OrthographicProjection::default_2d()
        }),
        Cursor::default(),
    ));
}

/// Cursor world position, relative to the camera.
#[derive(Component, Default)]
struct Cursor(Option<Vec2>);

// TODO Consider adding state `Exiting` that will send the AppExit Message. This will be benefitial when there is any other logic to do when exiting the game.
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
#[states(scoped_entities)]
enum MainState {
    #[default]
    Splash,
    Menu,
    Game,
}
