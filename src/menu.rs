use super::GameState;
use bevy::prelude::*;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), menu_enter)
        .add_systems(Update, menu_update.run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), menu_exit);
}

fn menu_enter(mut commands: Commands, mut camera_query: Query<&mut Transform, With<Camera2d>>) {
    let mut camera = camera_query.single_mut();
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
            parent.spawn((Button, Text::new("Game <Enter>")));
            parent.spawn((Button, Text::new("Settings <Esc>")));
            parent.spawn((Button, Text::new("Exit")));
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
