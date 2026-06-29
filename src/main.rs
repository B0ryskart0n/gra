mod assets;
mod game;
mod menu;
mod settings;
mod splash;
mod utils;

use bevy::camera::visibility::RenderLayers;
use bevy::camera::{RenderTarget, ScalingMode};
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy::window::WindowResized;

const RESOLUTION_WIDTH: u32 = 640;
const RESOLUTION_HEIGHT: u32 = 360;
const PIXELS_PER_METER: f32 = 16.0;
const METERS_PER_PIXEL: f32 = 1.0 / PIXELS_PER_METER;

/// Default `ImageCamera` pixel-perfect rendering layer.
const IMAGE_RENDERING_LAYER: RenderLayers = RenderLayers::layer(0);
/// `WindowCamera` normal rendering layer.
const WINDOW_RENDERING_LAYER: RenderLayers = RenderLayers::layer(1);

// TODO Consider using Events instead of using Messages everywhere.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        //.insert_resource(UiDebugOptions { enabled: true, ..default() }) // Draws debug borders of Nodes, requires `bevy_ui_debug` feature
        //.add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin::default())
        //.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
        // Needs to be done after StatesPlugin (part of DefaultPlugins)
        .init_state::<MainState>() // Initial state will be the #[default]
        .add_systems(Startup, startup)
        .add_systems(Update, fit_canvas)
        .add_plugins(splash::plugin)
        .add_plugins(menu::plugin)
        .add_plugins(settings::plugin)
        .add_plugins(game::game_plugin)
        .run();
}

fn startup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // The pixel-perfect image to be rendered onto the window. The game will be rendered onto this image.
    let canvas = images.add(Image::new_target_texture(
        RESOLUTION_WIDTH,
        RESOLUTION_HEIGHT,
        TextureFormat::Bgra8UnormSrgb,
        None,
    ));

    commands.spawn((
        ImageCamera,
        Camera2d,
        Cursor::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Srgba::gray(0.1).into()),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: RESOLUTION_WIDTH as f32 * METERS_PER_PIXEL,
                height: RESOLUTION_HEIGHT as f32 * METERS_PER_PIXEL,
            },
            ..OrthographicProjection::default_2d()
        }),
        RenderTarget::Image(canvas.clone().into()),
        Msaa::Off,
        IMAGE_RENDERING_LAYER,
    ));

    commands.spawn((Sprite::from_image(canvas), WINDOW_RENDERING_LAYER));
    commands.spawn((Camera2d, WindowCamera, WINDOW_RENDERING_LAYER));
}
// TODO
// A HUGE caveat to this approach to pixel-perfect scaling is that in window resolution the smallest possible movement is by N pixels (where N is the integer scaling), because the canvas contains no information apart from the pixels themselves.
fn fit_canvas(
    mut resize_messages: MessageReader<WindowResized>,
    mut q_camera: Query<&mut Projection, With<WindowCamera>>,
) -> Result {
    let mut projection = q_camera.single_mut()?;
    if let Projection::Orthographic(ref mut orthographic_projection) = *projection {
        if let Some(window_resized) = resize_messages.read().last() {
            let h_scale = window_resized.width / RESOLUTION_WIDTH as f32;
            let v_scale = window_resized.height / RESOLUTION_HEIGHT as f32;
            orthographic_projection.scale = 1. / h_scale.min(v_scale);
        }
    }
    Ok(())
}

// TODO Fix after moving to the canvas rendering.
/// Cursor world position, relative to the camera.
#[derive(Component, Default)]
struct Cursor(Option<Vec2>);
#[derive(Component)]
struct ImageCamera;
#[derive(Component)]
struct WindowCamera;

// TODO Consider adding state `Exiting` that will send the AppExit Message. This will be benefitial when there is any other logic to do when exiting the game.
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
#[states(scoped_entities)]
enum MainState {
    #[default]
    Splash,
    Menu,
    Game,
}
