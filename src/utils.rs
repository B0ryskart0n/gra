use bevy::prelude::*;
use std::f32::consts::FRAC_1_SQRT_2;

/// Rate of exponential decay in the distance between camera and Soldier
pub const CAMERA_DECAY_RATE: f32 = 3.0;

pub const DIRECTION_RIGHT: Vec2 = Vec2::X;
pub const DIRECTION_UPRIGHT: Vec2 = Vec2 {
    x: FRAC_1_SQRT_2,
    y: FRAC_1_SQRT_2,
};
pub const DIRECTION_UP: Vec2 = Vec2::Y;
pub const DIRECTION_UPLEFT: Vec2 = Vec2 {
    x: -FRAC_1_SQRT_2,
    y: FRAC_1_SQRT_2,
};
pub const DIRECTION_LEFT: Vec2 = Vec2::NEG_X;
pub const DIRECTION_DOWNLEFT: Vec2 = Vec2 {
    x: -FRAC_1_SQRT_2,
    y: -FRAC_1_SQRT_2,
};
pub const DIRECTION_DOWN: Vec2 = Vec2::NEG_Y;
pub const DIRECTION_DOWNRIGHT: Vec2 = Vec2 {
    x: FRAC_1_SQRT_2,
    y: -FRAC_1_SQRT_2,
};

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn _despawn<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
