use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy::ui::BackgroundColor;

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

#[derive(Component, Default)]
#[require(Node(init_parent_node))]
pub struct ParentNode;

fn init_parent_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Node::DEFAULT
    }
}
