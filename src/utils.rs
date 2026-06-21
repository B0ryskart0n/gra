pub mod ui {
    use bevy::prelude::*;

    pub fn typical_parent_node() -> Node {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            // Stack items from top to bottom
            flex_direction: FlexDirection::Column,
            // items will align to the center of the main axis
            justify_content: JustifyContent::Center,
            // items will (by default) align to center of the cross axis
            align_items: AlignItems::Center,
            ..Node::DEFAULT
        }
    }
}
