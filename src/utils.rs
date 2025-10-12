use bevy::prelude::*;

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn _despawn<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

pub fn square_collide(pos_a: Vec3, size_a: f32, pos_b: Vec3, size_b: f32) -> bool {
    return pos_a.x - size_a / 2.0 < pos_b.x + size_b / 2.0
        && pos_a.x + size_a / 2.0 > pos_b.x - size_b / 2.0
        && pos_a.y - size_a / 2.0 < pos_b.y + size_b / 2.0
        && pos_a.y + size_a / 2.0 > pos_b.y - size_b / 2.0;
}

#[derive(Component)]
pub struct Lifetime(pub Timer);
impl Lifetime {
    pub fn new(seconds: f32) -> Self {
        Lifetime(Timer::from_seconds(seconds, TimerMode::Once))
    }
}

pub fn lifetime(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    let dt = time.delta();
    query.iter_mut().for_each(|(e, mut l)| {
        if l.0.tick(dt).is_finished() {
            commands.entity(e).despawn()
        }
    })
}

pub mod ui {
    use bevy::ecs::query::QuerySingleError;
    use bevy::prelude::*;
    use bevy::ui::BackgroundColor;

    // TODO Consider, instead of taking a closure taking an EventWriter and handle that elsewhere.
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
