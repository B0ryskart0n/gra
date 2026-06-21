use super::MainState;
use crate::settings::UserSettings;
use crate::utils::ui;

// Prelude exports bevy_ui::widget::Button, but what I'm interested in is actually bevy::ui_widgets::Button
use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy::ui_widgets::Button;
use bevy::ui_widgets::observe;

pub fn plugin(app: &mut App) {
    app.add_sub_state::<MenuSubState>()
        .add_systems(OnEnter(MenuSubState::Main), main_ui)
        .add_systems(Update, handle_keyboard.run_if(in_state(MenuSubState::Main)))
        .add_systems(Update, update_interacted_buttons_display)
        .add_systems(OnEnter(MenuSubState::Settings), settings_ui)
        .add_systems(
            Update,
            (update_window_mode_text, update_resolution_text)
                .run_if(in_state(MenuSubState::Settings)),
        );
}

fn main_ui(mut commands: Commands) -> Result {
    commands
        .spawn((ui::typical_parent_node(), DespawnOnExit(MenuSubState::Main)))
        .with_children(|parent| {
            parent.spawn((
                MyButton,
                Text::new("Play"),
                observe(
                    |_: On<Activate>, mut next_state: ResMut<NextState<MainState>>| {
                        next_state.set(MainState::Game)
                    },
                ),
            ));
            parent.spawn((
                MyButton,
                Text::new("Settings"),
                observe(
                    |_: On<Activate>, mut next_substate: ResMut<NextState<MenuSubState>>| {
                        next_substate.set(MenuSubState::Settings)
                    },
                ),
            ));
            parent.spawn((
                MyButton,
                Text::new("Exit"),
                observe(
                    |_: On<Activate>, mut exit_messages: MessageWriter<AppExit>| {
                        exit_messages.write_default();
                    },
                ),
            ));
        });
    Ok(())
}
fn settings_ui(mut commands: Commands) {
    commands
        .spawn((
            ui::typical_parent_node(),
            DespawnOnExit(MenuSubState::Settings),
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
                                MyButton,
                                Text::new("Resolution"),
                                observe(
                                    |_: On<Activate>, mut user_settings: ResMut<UserSettings>| {
                                        user_settings.window.cycle_res()
                                    },
                                ),
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
                                MyButton,
                                Text::new("Window Mode"),
                                observe(
                                    |_: On<Activate>, mut user_settings: ResMut<UserSettings>| {
                                        user_settings.window.cycle_mode()
                                    },
                                ),
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
                    parent.spawn((
                        MyButton,
                        Text::new("Menu"),
                        observe(
                            |_: On<Activate>, mut next_substate: ResMut<NextState<MenuSubState>>| {
                                next_substate.set(MenuSubState::Main)
                            },
                        ),
                    ));
                    parent.spawn((
                        MyButton,
                        Text::new("Apply"),
                        observe(
                            |_: On<Activate>,
                             mut q_window: Query<&mut Window>,
                             user_settings: Res<UserSettings>| {
                                let mut window =
                                    q_window.single_mut().expect("there should be one window");
                                user_settings.window.update_bevy_window(&mut window);
                            },
                        ),
                    ));
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

fn update_interacted_buttons_display(
    mut q_buttons: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    q_buttons.iter_mut().for_each(|(interaction, mut color)| {
        // if let Ok((interaction, mut color)) = button_query_result {
        *color = match interaction {
            Interaction::None => BackgroundColor::DEFAULT,
            Interaction::Hovered => BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
            Interaction::Pressed => BackgroundColor(Color::srgb(0.5, 1.0, 0.5)),
        };
    });
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
#[require(Interaction, Button, BackgroundColor)]
struct MyButton;

#[derive(Component)]
struct ResolutionText;
#[derive(Component)]
struct WindowModeText;
