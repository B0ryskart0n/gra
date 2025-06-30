use crate::MainState;
use crate::utils::ui::*;

use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::window::WindowResolution;

const LOGICAL_WIDTH: u16 = 640;
const LOGICAL_HEIGHT: u16 = 360;

pub fn plugin(app: &mut App) {
    app.init_resource::<UserSettings>()
        .add_systems(PreStartup, startup_window_settings)
        .add_systems(OnEnter(MainState::Settings), setup_ui)
        .add_systems(
            Update,
            (
                update_text_mode,
                update_text_res,
                handle_keyboard,
                handle_menu_button,
                handle_apply_button,
                handle_resolution_button,
                handle_window_mode_button,
            )
                .run_if(in_state(MainState::Settings)),
        );
}

fn startup_window_settings(
    mut q_window: Query<&mut Window>,
    settings: Res<UserSettings>,
) -> Result {
    let mut window = q_window.single_mut()?;
    window.decorations = false;
    window.resolution = settings.window.to_bevy_res();
    Ok(())
}

fn setup_ui(mut commands: Commands) {
    commands
        .spawn((typical_parent_node(), StateScoped(MainState::Settings)))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(80.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node {
                            width: Val::Percent(40.0),
                            height: Val::Auto,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Default,
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn((ResolutionButton, Text::new("Resolution")));
                            parent.spawn((ResolutionText, Text::default()));
                        });
                    parent
                        .spawn(Node {
                            width: Val::Percent(40.0),
                            height: Val::Auto,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Default,
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn((WindowModeButton, Text::new("Window Mode")));
                            parent.spawn((WindowModeText, Text::default()));
                        });
                });
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceEvenly,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((MenuButton, Text::new("Menu")));
                    parent.spawn((ApplyButton, Text::new("Apply")));
                });
        });
}
fn update_text_mode(
    mut q_window_mode: Query<&mut Text, With<WindowModeText>>,
    user_settings: Res<UserSettings>,
) -> Result {
    q_window_mode.single_mut()?.0 = user_settings.window.mode_str();
    Ok(())
}
fn update_text_res(
    mut q_resolution: Query<&mut Text, With<ResolutionText>>,
    user_settings: Res<UserSettings>,
) -> Result {
    q_resolution.single_mut()?.0 = user_settings.window.res_str();
    Ok(())
}
fn handle_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MainState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(MainState::Menu);
    }
}
fn handle_menu_button(
    mut q_button: Query<
        (&Interaction, &mut BackgroundColor),
        (With<MenuButton>, Changed<Interaction>),
    >,
    mut next_state: ResMut<NextState<MainState>>,
) {
    button_interaction(q_button.single_mut(), || next_state.set(MainState::Menu));
}
// TODO Solve with Events and handle also pressing 'Enter'.
// TODO There is no coming back from setting the resolution too big,
// let's introduce a mechanism to fallback to previous settings.
fn handle_apply_button(
    mut q_button: Query<
        (&Interaction, &mut BackgroundColor),
        (With<ApplyButton>, Changed<Interaction>),
    >,
    mut q_window: Query<&mut Window>,
    user_settings: Res<UserSettings>,
) -> Result {
    let mut window = q_window.single_mut()?;
    button_interaction(q_button.single_mut(), || {
        // Setting the mode before the resolution seems to work better.
        window.mode = user_settings.window.to_bevy_mode();
        window.resolution = user_settings.window.to_bevy_res();
    });

    Ok(())
}
fn handle_resolution_button(
    mut q_button: Query<
        (&Interaction, &mut BackgroundColor),
        (With<ResolutionButton>, Changed<Interaction>),
    >,
    mut user_settings: ResMut<UserSettings>,
) {
    button_interaction(q_button.single_mut(), || user_settings.window.cycle_res());
}
fn handle_window_mode_button(
    mut q_button: Query<
        (&Interaction, &mut BackgroundColor),
        (With<WindowModeButton>, Changed<Interaction>),
    >,
    mut user_settings: ResMut<UserSettings>,
) {
    button_interaction(q_button.single_mut(), || user_settings.window.cycle_mode());
}

// TODO Add loading last settings and falling back to creating defaults from system settings.
// Also synchronize with bevy settings, since initial values will not match real ones.
#[derive(Debug, Resource, Default)]
pub struct UserSettings {
    pub window: WindowSettings,
}
// TODO Take Monitor information into account when determining resoltuions.
#[derive(Debug)]
pub enum WindowSettings {
    Windowed(Resolution),
    Borderless,
    Fullscreen,
}
impl Default for WindowSettings {
    fn default() -> Self {
        Self::Windowed(Resolution::Logical)
    }
}
impl WindowSettings {
    fn to_bevy_mode(&self) -> WindowMode {
        match self {
            Self::Windowed(_) => WindowMode::Windowed,
            Self::Borderless => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            Self::Fullscreen => {
                WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current)
            }
        }
    }
    fn to_bevy_res(&self) -> WindowResolution {
        match self {
            WindowSettings::Windowed(res) => {
                let mut window_resolution = WindowResolution::from(res.pixels());
                window_resolution.set_scale_factor(res.scale());
                window_resolution
            }
            _ => WindowResolution::from(Resolution::FullHD.pixels())
                .with_scale_factor_override(Resolution::FullHD.scale()),
        }
    }
    fn cycle_mode(&mut self) {
        *self = match self {
            Self::Windowed(_) => Self::Borderless,
            Self::Borderless => Self::Fullscreen,
            Self::Fullscreen => Self::default(),
        }
    }
    fn cycle_res(&mut self) {
        match self {
            Self::Windowed(res) => res.cycle(),
            _ => (),
        }
    }
    fn mode_str(&self) -> String {
        match self {
            Self::Windowed(_) => "Windowed".into(),
            Self::Borderless => "Borderless".into(),
            Self::Fullscreen => "Fullscreen".into(),
        }
    }
    fn res_str(&self) -> String {
        match self {
            Self::Windowed(res) => res.to_string(),
            _ => Resolution::FullHD.to_string(),
        }
    }
}

#[derive(Default, Debug)]
pub enum Resolution {
    #[default]
    Logical,
    HD,
    FullHD,
    QHD,
}
impl Resolution {
    fn scale(&self) -> f32 {
        match self {
            Self::Logical => 1.0,
            Self::HD => 2.0,
            Self::FullHD => 3.0,
            Self::QHD => 4.0,
        }
    }
    fn pixels(&self) -> [u16; 2] {
        match self {
            Self::Logical => [LOGICAL_WIDTH, LOGICAL_HEIGHT],
            Self::HD => [1280, 720],
            Self::FullHD => [1920, 1080],
            Self::QHD => [2560, 1440],
        }
    }
    fn cycle(&mut self) {
        *self = match self {
            Self::Logical => Self::HD,
            Self::HD => Self::FullHD,
            Self::FullHD => Self::QHD,
            Self::QHD => Self::Logical,
        };
    }
}
impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} * {}", self.pixels()[0], self.pixels()[1])
    }
}

#[derive(Component)]
#[require(ButtonWithBackground)]
struct ApplyButton;
#[derive(Component)]
#[require(ButtonWithBackground)]
struct MenuButton;
#[derive(Component)]
#[require(ButtonWithBackground)]
struct ResolutionButton;
#[derive(Component)]
#[require(ButtonWithBackground)]
struct WindowModeButton;

#[derive(Component)]
struct ResolutionText;
#[derive(Component)]
struct WindowModeText;
