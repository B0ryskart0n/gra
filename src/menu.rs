use crate::settings::UserSettings;
use crate::utils::ui;

// Prelude exports bevy_ui::widget::Button, but what I'm interested in is actually bevy::ui_widgets::Button
use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy::ui_widgets::Button;
use bevy::ui_widgets::observe;

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
                        observe(
                            |_: On<Activate>,
                             q_window: Query<&mut Window>,
                             user_settings: Res<UserSettings>| {
                                info!("Apply");
                                user_settings.apply_settings(q_window);
                            },
                        ),
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
