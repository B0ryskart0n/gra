use super::MainState;
use crate::utils::ui::*;
use bevy::prelude::*;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(MainState::Menu), setup_ui)
        .add_systems(
            Update,
            (
                handle_keyboard,
                handle_game_button,
                handle_settings_button,
                handle_exit_button,
            )
                .run_if(in_state(MainState::Menu)),
        );
}

fn setup_ui(mut commands: Commands) -> Result {
    commands
        .spawn((typical_parent_node(), DespawnOnExit(MainState::Menu)))
        .with_children(|parent| {
            parent.spawn((GameButton, Text::new("Play")));
            parent.spawn((SettingsButton, Text::new("Settings")));
            parent.spawn((ExitButton, Text::new("Exit")));
        });
    Ok(())
}

fn handle_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MainState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(MainState::Game);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(MainState::Settings);
    }
}

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
    mut next_state: ResMut<NextState<MainState>>,
) {
    button_interaction(q_button.single_mut(), || {
        next_state.set(MainState::Settings)
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

#[derive(Component)]
#[require(ButtonWithBackground)]
struct GameButton;
#[derive(Component)]
#[require(ButtonWithBackground)]
struct SettingsButton;
#[derive(Component)]
#[require(ButtonWithBackground)]
struct ExitButton;
