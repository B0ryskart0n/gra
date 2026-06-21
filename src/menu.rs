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
        .add_systems(Update, update_interacted_buttons_display)
        .add_systems(OnEnter(MenuSubState::Settings), settings_ui);
}

fn main_ui(mut commands: Commands) -> Result {
    commands
        .spawn((ui::typical_parent_node(), DespawnOnExit(MenuSubState::Main)))
        .with_children(|parent| {
            parent.spawn((
                MyButton,
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
            parent.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(80.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..Default::default()
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
                                info!("Menu");
                                next_substate.set(MenuSubState::Main)
                            },
                        ),
                    ));
                    parent.spawn((
                        MyButton,
                        Text::new("Apply"),
                        observe(|_: On<Activate>, mut q_window: Query<&mut Window>| {
                            info!("Apply");
                            let mut bevy_window =
                                q_window.single_mut().expect("expected exactly one window");
                            bevy_window.mode = WindowMode::Windowed;
                            bevy_window
                                .resolution
                                .set_physical_resolution(HD_WIDTH, HD_HEIGHT);
                            bevy_window.resolution.set_scale_factor(2.0);
                        }),
                    ));
                });
        });
}

fn update_interacted_buttons_display(
    mut q_buttons: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    q_buttons.iter_mut().for_each(|(interaction, mut color)| {
        *color = match interaction {
            Interaction::None => BackgroundColor::DEFAULT,
            Interaction::Hovered => BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
            Interaction::Pressed => BackgroundColor(Color::srgb(0.5, 1.0, 0.5)),
        };
    });
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuSubState {
    #[default]
    Main,
    Settings,
}

#[derive(Component)]
#[require(Interaction, Button, BackgroundColor)]
struct MyButton;
