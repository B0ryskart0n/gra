use super::GameState;
use crate::ui::*;
use bevy::prelude::*;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), setup_ui).add_systems(
        Update,
        (
            handle_keyboard,
            handle_game_button,
            handle_settings_button,
            handle_exit_button,
        )
            .run_if(in_state(GameState::Menu)),
    );
}

fn setup_ui(mut commands: Commands, mut camera_query: Query<&mut Transform, With<Camera2d>>) {
    let mut camera = camera_query.single_mut();
    // TODO Consider whether this should be part of menu or game logic.
    camera.translation = Vec3::ZERO;

    commands
        .spawn((typical_parent_node(), StateScoped(GameState::Menu)))
        .with_children(|parent| {
            parent.spawn((GameButton, Text::new("Play")));
            parent.spawn((SettingsButton, Text::new("Settings")));
            parent.spawn((ExitButton, Text::new("Exit")));
        });
}

fn handle_keyboard(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Game);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Settings);
    }
}

fn handle_game_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), (With<GameButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    button_interaction(q_button.get_single_mut(), || next_state.set(GameState::Game));
}
fn handle_settings_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), With<SettingsButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    button_interaction(q_button.get_single_mut(), || next_state.set(GameState::Settings));
}
fn handle_exit_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), With<ExitButton>>,
    mut exit_events: EventWriter<AppExit>,
) {
    button_interaction(q_button.get_single_mut(), || {
        exit_events.send_default();
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
