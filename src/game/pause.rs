use super::*;
use crate::MainState;
use bevy::prelude::*;

pub fn toggle(
    mut time: ResMut<Time<Virtual>>,
    current_state: Res<State<GameSubState>>,
    mut next_state: ResMut<NextState<GameSubState>>,
    mut q_overlay: Query<&mut Visibility, With<PauseOverlay>>,
) -> Result {
    let mut overlay_visibility = q_overlay.single_mut()?;
    match current_state.get() {
        GameSubState::Running => {
            time.pause();
            *overlay_visibility = Visibility::Visible;
            next_state.set(GameSubState::Paused);
        }
        GameSubState::Paused => {
            time.unpause();
            *overlay_visibility = Visibility::Hidden;
            next_state.set(GameSubState::Running);
        }
    }
    Ok(())
}

pub fn spawn_invisible_overlay(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Node::DEFAULT
            },
            DespawnOnExit(MainState::Game),
            BackgroundColor(Color::srgba_u8(0, 0, 0, 200)),
            Visibility::Hidden,
            PauseOverlay,
        ))
        .with_child(Text::new("Paused"));
}

#[derive(Component)]
pub struct PauseOverlay;
