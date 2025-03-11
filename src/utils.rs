use bevy::prelude::*;
use std::f32::consts::FRAC_1_SQRT_2;

pub const ZERO3: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

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
        commands.entity(entity).despawn_recursive();
    }
}
