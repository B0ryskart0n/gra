use bevy::prelude::*;

mod assets;
mod game;
mod menu;
mod settings;
mod splash;
mod utils;

// TODO Consider using Events instead of using Messages everywhere.
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                // FIXME I suspect there is a bug in wayland/hyprland and changing the resolution after window creation is problematic.
                // Therefore set the resolution here to a reasonable value to work with.
                // Let's leave this issue for now. Maybe it will get fixed in the meantime.
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: bevy::window::WindowResolution::new(1280, 720)
                            .with_scale_factor_override(2.0),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        //.add_plugins(bevy::dev_tools::fps_overlay::FpsOverlayPlugin::default())
        .add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin::default())
        .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
        // Needs to be done after StatesPlugin (part of DefaultPlugins)
        .init_state::<MainState>() // Initial state will be the #[default]
        .add_systems(Startup, spawn_camera)
        .add_plugins(splash::plugin)
        .add_plugins(menu::plugin)
        .add_plugins(settings::plugin)
        .add_plugins(game::game_plugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((PrimaryCamera, Cursor::default()));
}

/// Cursor world position, relative to the camera.
#[derive(Component, Default)]
struct Cursor(Option<Vec2>);
#[derive(Component)]
#[require(Camera2d)]
struct PrimaryCamera;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
#[states(scoped_entities)]
enum MainState {
    #[default]
    Splash,
    Menu,
    Game,
}
