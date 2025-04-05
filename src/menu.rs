use super::GameState;
use bevy::prelude::*;

const BUTTON_HOVERED_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const BUTTON_PRESSED_COLOR: Color = Color::srgb(0.5, 1.0, 0.5);

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), menu_enter)
        .add_systems(
            Update,
            (
                menu_update,
                handle_game_button,
                handle_settings_button,
                handle_exit_button,
            )
                .run_if(in_state(GameState::Menu)),
        )
        .add_systems(OnExit(GameState::Menu), menu_exit);
}

fn menu_enter(mut commands: Commands, mut camera_query: Query<&mut Transform, With<Camera2d>>) {
    let mut camera = camera_query.single_mut();
    // TODO Consider whether this should be part of menu or game logic.
    camera.translation = Vec3::ZERO;

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
            StateScoped(GameState::Menu),
        ))
        .with_children(|parent| {
            // TODO Create a component that is essentially `Button` with hover and press functions?
            parent.spawn((Button, BackgroundColor::DEFAULT, Text::new("Play"), GameButton));
            parent.spawn((Button, BackgroundColor::DEFAULT, Text::new("Settings"), SettingsButton));
            parent.spawn((Button, BackgroundColor::DEFAULT, Text::new("Exit"), ExitButton));
        });
}
fn menu_exit() {}

fn menu_update(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Game);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Settings);
    }
}

// TODO Maybe it would be faster to filter for `Changed<Interaction`,
// but that would be more code and the performance gain should be negligable.
fn handle_game_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), With<GameButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (interaction, mut color) = q_button.single_mut();
    *color = match interaction {
        Interaction::None => BackgroundColor::DEFAULT,
        Interaction::Hovered => BackgroundColor(BUTTON_HOVERED_COLOR),
        Interaction::Pressed => {
            next_state.set(GameState::Game);
            BackgroundColor(BUTTON_PRESSED_COLOR)
        }
    }
}
fn handle_settings_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), With<SettingsButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (interaction, mut color) = q_button.single_mut();
    *color = match interaction {
        Interaction::None => BackgroundColor::DEFAULT,
        Interaction::Hovered => BackgroundColor(BUTTON_HOVERED_COLOR),
        Interaction::Pressed => {
            next_state.set(GameState::Settings);
            BackgroundColor(BUTTON_PRESSED_COLOR)
        }
    }
}
fn handle_exit_button(
    mut q_button: Query<(&Interaction, &mut BackgroundColor), With<ExitButton>>,
    mut exit_events: EventWriter<AppExit>,
) {
    let (interaction, mut color) = q_button.single_mut();
    *color = match interaction {
        Interaction::None => BackgroundColor::DEFAULT,
        Interaction::Hovered => BackgroundColor(BUTTON_HOVERED_COLOR),
        Interaction::Pressed => {
            exit_events.send_default();
            BackgroundColor(BUTTON_PRESSED_COLOR)
        }
    }
}

#[derive(Component)]
struct GameButton;
#[derive(Component)]
struct SettingsButton;
#[derive(Component)]
struct ExitButton;
