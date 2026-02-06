use bevy::prelude::*;

mod assets;
mod game;
mod menu;
mod settings;
mod splash;
mod utils;

// Important for perspective and sprite scaling
const PIXELS_PER_METER: f32 = 16.0;
const METERS_PER_PIXEL: f32 = 1.0 / PIXELS_PER_METER;

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

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
#[states(scoped_entities)]
enum MainState {
    #[default]
    Splash,
    Menu,
    Game,
}
