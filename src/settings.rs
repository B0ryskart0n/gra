use crate::GameState;
use crate::ui::*;
use bevy::prelude::*;
use bevy::window::WindowMode;

pub fn plugin(app: &mut App) {
    app.init_resource::<UserSettings>()
        .add_systems(OnEnter(GameState::Settings), enter_settings)
        .add_systems(
            Update,
            (
                update_texts,
                handle_keyboard,
                handle_menu_button,
                handle_apply_button,
                handle_resolution_button,
                handle_window_mode_button,
            )
                .run_if(in_state(GameState::Settings)),
        );
}

fn enter_settings(mut commands: Commands) {
    commands
        .spawn((typical_parent_node(), StateScoped(GameState::Settings)))
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
fn update_texts(
    mut q_resolution: Query<&mut Text, With<ResolutionText>>,
    mut q_window_mode: Query<&mut Text, (With<WindowModeText>, Without<ResolutionText>)>,
    user_settings: Res<UserSettings>,
) {
    q_resolution.single_mut().0 = user_settings.resolution.to_string();
    q_window_mode.single_mut().0 = user_settings.window_mode.to_string();
}
fn handle_keyboard(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}
fn handle_menu_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), (With<MenuButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    button_interaction(q_button.get_single_mut(), || next_state.set(GameState::Menu));
}
fn handle_apply_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), (With<ApplyButton>, Changed<Interaction>)>,
    mut q_window: Query<&mut Window, Without<ApplyButton>>,
    user_settings: Res<UserSettings>,
) {
    button_interaction(q_button.get_single_mut(), || {
        let mut window = q_window.single_mut();
        // TODO Problems in non-windowed mode. Experiment what happens when only window_mode is changed.
        // Setting the mode before the resolution seems to work better.
        window.mode = user_settings.window_mode.to_bevy();
        let pixels = user_settings.resolution.pixels();
        window.resolution.set(pixels[0].into(), pixels[1].into());
    });
}
fn handle_resolution_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), (With<ResolutionButton>, Changed<Interaction>)>,
    mut user_settings: ResMut<UserSettings>,
) {
    button_interaction(q_button.get_single_mut(), || user_settings.resolution.cycle());
}
fn handle_window_mode_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), (With<WindowModeButton>, Changed<Interaction>)>,
    mut user_settings: ResMut<UserSettings>,
) {
    button_interaction(q_button.get_single_mut(), || user_settings.window_mode.cycle());
}

// TODO Add loading last settings and falling back to creating defaults from system settings.
// Also synchronize with bevy settings, since initial values will not match real ones.
#[derive(Debug, Resource, Default)]
pub struct UserSettings {
    pub resolution: Resolution,
    pub window_mode: MyWindowMode,
}
#[derive(Debug, Default)]
pub enum MyWindowMode {
    #[default]
    Windowed,
    Borderless,
    Fullscreen,
}
impl MyWindowMode {
    fn to_bevy(&self) -> WindowMode {
        match self {
            Self::Windowed => WindowMode::Windowed,
            Self::Borderless => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            Self::Fullscreen => WindowMode::Fullscreen(MonitorSelection::Current),
        }
    }
    fn cycle(&mut self) {
        *self = match self {
            Self::Windowed => Self::Borderless,
            Self::Borderless => Self::Fullscreen,
            Self::Fullscreen => Self::Windowed,
        }
    }
}
impl std::fmt::Display for MyWindowMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default, Debug)]
pub enum Resolution {
    #[default]
    HD,
    TestingRes,
    FullHD,
    QHD,
}
impl Resolution {
    fn pixels(&self) -> [u16; 2] {
        match self {
            Self::HD => [1280, 720],
            Self::TestingRes => [1600, 900],
            Self::FullHD => [1920, 1080],
            Self::QHD => [2560, 1440],
        }
    }
    fn cycle(&mut self) {
        *self = match self {
            Self::HD => Self::TestingRes,
            Self::TestingRes => Self::FullHD,
            Self::FullHD => Self::QHD,
            Self::QHD => Self::HD,
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
