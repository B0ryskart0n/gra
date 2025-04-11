use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy::ui::BackgroundColor;

// TODO Instead of taking a closure it migt be good to give it an EventWriter and handle that elsewhere.
/// Takes the Result of `.get_single_mut` called on a query for Button that should be updated
/// based on the interaction and the closure to call is it is pressed.
pub fn button_interaction(
    button_query_result: Result<(&Interaction, Mut<BackgroundColor>), QuerySingleError>,
    pressed: impl FnOnce(),
) {
    if let Ok((interaction, mut color)) = button_query_result {
        *color = match interaction {
            Interaction::None => BackgroundColor::DEFAULT,
            Interaction::Hovered => BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
            Interaction::Pressed => {
                pressed();
                BackgroundColor(Color::srgb(0.5, 1.0, 0.5))
            }
        };
    }
}

#[derive(Component, Default)]
#[require(Button, BackgroundColor)]
pub struct ButtonWithBackground;

pub fn typical_parent_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Node::DEFAULT
    }
}
