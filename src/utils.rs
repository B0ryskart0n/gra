use crate::PrimaryCamera;
use bevy::prelude::*;
use std::f32::consts::FRAC_1_SQRT_2;

pub const DIRECTION_RIGHT: Vec3 = Vec3::X;
pub const DIRECTION_UPRIGHT: Vec3 = Vec3 {
    x: FRAC_1_SQRT_2,
    y: FRAC_1_SQRT_2,
    z: 0.0,
};
pub const DIRECTION_UP: Vec3 = Vec3::Y;
pub const DIRECTION_UPLEFT: Vec3 = Vec3 {
    x: -FRAC_1_SQRT_2,
    y: FRAC_1_SQRT_2,
    z: 0.0,
};
pub const DIRECTION_LEFT: Vec3 = Vec3::NEG_X;
pub const DIRECTION_DOWNLEFT: Vec3 = Vec3 {
    x: -FRAC_1_SQRT_2,
    y: -FRAC_1_SQRT_2,
    z: 0.0,
};
pub const DIRECTION_DOWN: Vec3 = Vec3::NEG_Y;
pub const DIRECTION_DOWNRIGHT: Vec3 = Vec3 {
    x: FRAC_1_SQRT_2,
    y: -FRAC_1_SQRT_2,
    z: 0.0,
};

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn _despawn<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

pub fn reset_camera(mut q_camera: Query<&mut Transform, With<PrimaryCamera>>) -> Result {
    q_camera.single_mut()?.translation = Vec3::ZERO;
    Ok(())
}

pub mod ui {
    use bevy::ecs::query::QuerySingleError;
    use bevy::prelude::*;
    use bevy::ui::BackgroundColor;

    // TODO Instead of taking a closure it migt be good to give it an EventWriter and handle that elsewhere.
    /// Takes the Result of `.single_mut` called on a query for Button that should be updated
    /// based on the interaction, and the closure to call is it is pressed.
    /// Only works with `Result::Ok` variant, assuming the error means empty query caused by no `Changed<Interaction>`
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
}
