use super::MainState;
use crate::settings::UserSettings;
use crate::utils::ui::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_sub_state::<MenuSubState>()
        .add_systems(OnEnter(MenuSubState::Main), main_ui)
        .add_systems(
            Update,
            (
                handle_keyboard,
                handle_game_button,
                handle_settings_button,
                handle_exit_button,
            )
                .run_if(in_state(MenuSubState::Main)),
        )
        .add_systems(OnEnter(MenuSubState::Settings), settings_ui)
        .add_systems(
            Update,
            (
                update_window_mode_text,
                update_resolution_text,
                handle_menu_button,
                handle_apply_button,
                handle_resolution_button,
                handle_window_mode_button,
            )
                .run_if(in_state(MenuSubState::Settings)),
        );
}

fn main_ui(mut commands: Commands) -> Result {
    commands
        .spawn((typical_parent_node(), DespawnOnExit(MenuSubState::Main)))
        .with_children(|parent| {
            parent.spawn((GameButton, Text::new("Play")));
            parent.spawn((SettingsButton, Text::new("Settings")));
            parent.spawn((ExitButton, Text::new("Exit")));
        });
    Ok(())
}
fn settings_ui(mut commands: Commands) {
    commands
        .spawn((typical_parent_node(), DespawnOnExit(MenuSubState::Settings)))
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

// Meant only for the main menu, should not be run in sub-menus.
fn handle_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MainState>>,
    mut next_substate: ResMut<NextState<MenuSubState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(MainState::Game);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        next_substate.set(MenuSubState::Settings);
    }
}

// TODO Consider tying those functions with specific Buttons and running a big system
// that queries all buttons and processes their statuses.
fn handle_game_button(
    mut q_button: Query<
        (&Interaction, &mut BackgroundColor),
        (With<GameButton>, Changed<Interaction>),
    >,
    mut next_state: ResMut<NextState<MainState>>,
) -> Result {
    button_interaction(q_button.single_mut(), || next_state.set(MainState::Game));
    Ok(())
}
fn handle_settings_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), With<SettingsButton>>,
    mut next_substate: ResMut<NextState<MenuSubState>>,
) {
    button_interaction(q_button.single_mut(), || {
        next_substate.set(MenuSubState::Settings)
    });
}
fn handle_menu_button(
    mut q_button: Query<
        (&Interaction, &mut BackgroundColor),
        (With<MenuButton>, Changed<Interaction>),
    >,
    mut next_substate: ResMut<NextState<MenuSubState>>,
) {
    button_interaction(q_button.single_mut(), || {
        next_substate.set(MenuSubState::Main)
    });
}
fn handle_exit_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), With<ExitButton>>,
    mut exit_messages: MessageWriter<AppExit>,
) {
    button_interaction(q_button.single_mut(), || {
        exit_messages.write_default();
    });
}
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
        user_settings.window.set_bevy(&mut window);
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

fn update_window_mode_text(
    mut q_window_mode: Query<&mut Text, With<WindowModeText>>,
    user_settings: Res<UserSettings>,
) -> Result {
    q_window_mode.single_mut()?.0 = user_settings.window.mode_str();
    Ok(())
}
fn update_resolution_text(
    mut q_resolution: Query<&mut Text, With<ResolutionText>>,
    user_settings: Res<UserSettings>,
) -> Result {
    q_resolution.single_mut()?.0 = user_settings.window.res_str();
    Ok(())
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, SubStates)]
#[source(MainState = MainState::Menu)]
enum MenuSubState {
    #[default]
    Main,
    Settings,
}

#[derive(Component)]
#[require(ButtonWithBackground)]
struct GameButton;
#[derive(Component)]
#[require(ButtonWithBackground)]
struct SettingsButton;
#[derive(Component)]
#[require(ButtonWithBackground)]
struct ExitButton;
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
