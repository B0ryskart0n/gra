use crate::GameState;
use crate::utils::*;
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
        )
        .add_systems(OnExit(GameState::Settings), exit_settings);
}

fn enter_settings(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            StateScoped(GameState::Settings),
        ))
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
                            parent.spawn((
                                Button,
                                BackgroundColor::DEFAULT,
                                Text::new("Resolution"),
                                ResolutionButton,
                            ));
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
                            parent.spawn((
                                Button,
                                BackgroundColor::DEFAULT,
                                Text::new("Window Mode"),
                                WindowModeButton,
                            ));
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
                    parent.spawn((Button, BackgroundColor::DEFAULT, Text::new("Menu"), MenuButton));
                    parent.spawn((Button, BackgroundColor::DEFAULT, Text::new("Apply"), ApplyButton));
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
fn exit_settings() {}
fn handle_menu_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), With<MenuButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (interaction, mut color) = q_button.single_mut();
    *color = match interaction {
        Interaction::None => BackgroundColor::DEFAULT,
        Interaction::Hovered => BackgroundColor(BUTTON_HOVERED_COLOR),
        Interaction::Pressed => {
            next_state.set(GameState::Menu);
            BackgroundColor(BUTTON_PRESSED_COLOR)
        }
    }
}
// TODO Problems in non-windowed mode. Experiment what happens when only window_mode is changed.
fn handle_apply_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), With<ApplyButton>>,
    mut q_window: Query<&mut Window, Without<ApplyButton>>,
    user_settings: Res<UserSettings>,
) {
    let (interaction, mut color) = q_button.single_mut();
    *color = match interaction {
        Interaction::None => BackgroundColor::DEFAULT,
        Interaction::Hovered => BackgroundColor(BUTTON_HOVERED_COLOR),
        Interaction::Pressed => {
            let mut window = q_window.single_mut();
            // Setting the mode before the resolution seems to work better.
            window.mode = user_settings.window_mode.to_bevy();
            let pixels = user_settings.resolution.pixels();
            window.resolution.set(pixels[0].into(), pixels[1].into());
            BackgroundColor(BUTTON_PRESSED_COLOR)
        }
    }
}
/// Does NOT fail if the single entity was not found.
fn handle_resolution_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), (With<ResolutionButton>, Changed<Interaction>)>,
    mut user_settings: ResMut<UserSettings>,
) {
    if let Ok((interaction, mut color)) = q_button.get_single_mut() {
        *color = match interaction {
            Interaction::None => BackgroundColor::DEFAULT,
            Interaction::Hovered => BackgroundColor(BUTTON_HOVERED_COLOR),
            Interaction::Pressed => {
                user_settings.resolution.cycle();
                BackgroundColor(BUTTON_PRESSED_COLOR)
            }
        }
    }
}
/// Does NOT fail if the single entity was not found.
fn handle_window_mode_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), (With<WindowModeButton>, Changed<Interaction>)>,
    mut user_settings: ResMut<UserSettings>,
) {
    if let Ok((interaction, mut color)) = q_button.get_single_mut() {
        *color = match interaction {
            Interaction::None => BackgroundColor::DEFAULT,
            Interaction::Hovered => BackgroundColor(BUTTON_HOVERED_COLOR),
            Interaction::Pressed => {
                user_settings.window_mode.cycle();
                BackgroundColor(BUTTON_PRESSED_COLOR)
            }
        }
    }
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
            Self::TestingRes => [1664, 936],
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
struct ApplyButton;
#[derive(Component)]
struct MenuButton;
#[derive(Component)]
struct ResolutionButton;
#[derive(Component)]
struct WindowModeButton;
#[derive(Component)]
struct ResolutionText;
#[derive(Component)]
struct WindowModeText;
