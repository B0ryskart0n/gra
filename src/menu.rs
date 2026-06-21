use crate::utils::ui;

const HD_WIDTH: u32 = 1280;
const HD_HEIGHT: u32 = 720;
// Prelude exports bevy_ui::widget::Button, but what I'm interested in is actually bevy::ui_widgets::Button
use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy::ui_widgets::Button;
use bevy::ui_widgets::observe;
use bevy::window::WindowMode;

pub fn plugin(app: &mut App) {
    app.init_state::<MenuSubState>()
        .add_systems(OnEnter(MenuSubState::Main), main_ui)
        .add_systems(OnEnter(MenuSubState::Settings), settings_ui);
}

fn main_ui(mut commands: Commands) -> Result {
    commands
        .spawn((ui::typical_parent_node(), DespawnOnExit(MenuSubState::Main)))
        .with_children(|parent| {
            parent.spawn((
                Button,
                Text::new("Settings"),
                observe(
                    |_: On<Activate>, mut next_substate: ResMut<NextState<MenuSubState>>| {
                        info!("Settings");
                        next_substate.set(MenuSubState::Settings)
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
            parent.spawn((
                Button,
                Text::new("Menu"),
                observe(
                    |_: On<Activate>, mut next_substate: ResMut<NextState<MenuSubState>>| {
                        info!("Menu");
                        next_substate.set(MenuSubState::Main)
                    },
                ),
            ));
            parent.spawn((
                Button,
                Text::new("Apply"),
                observe(|_: On<Activate>, mut q_window: Query<&mut Window>| {
                    info!("Apply");
                    let mut bevy_window =
                        q_window.single_mut().expect("expected exactly one window");
                    bevy_window.mode = WindowMode::Windowed;
                    bevy_window
                        .resolution
                        .set_physical_resolution(HD_WIDTH, HD_HEIGHT);
                    bevy_window.resolution.set_scale_factor(3.0);
                }),
            ));
        });
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuSubState {
    Main,
    #[default]
    Settings,
}
