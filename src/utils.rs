use bevy::prelude::*;

pub fn image_size_to_sprite(image: Handle<Image>, meter_size: Vec2) -> Sprite {
    Sprite {
        image: image,
        custom_size: Some(meter_size),
        ..default()
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn _despawn<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
pub struct Lifetime(pub Timer);
impl Lifetime {
    pub fn new(seconds: f32) -> Self {
        Lifetime(Timer::from_seconds(seconds, TimerMode::Once))
    }
    /// Despawns entities which are past their bedtime.
    pub fn system(
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
}
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
