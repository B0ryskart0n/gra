#![allow(dead_code)]

use bevy::color::palettes::css::{BLUE, GREEN, RED};
use bevy::prelude::*;

#[derive(Resource)]
pub struct BasicColorHandles {
    pub red: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
    pub blue: Handle<ColorMaterial>,
    pub grey: Handle<ColorMaterial>,
}
impl BasicColorHandles {
    pub fn init_simple_colors(mut materials: ResMut<Assets<ColorMaterial>>) -> Self {
        BasicColorHandles {
            red: materials.add(ColorMaterial::from_color(RED)),
            green: materials.add(ColorMaterial::from_color(GREEN)),
            blue: materials.add(ColorMaterial::from_color(BLUE)),
            grey: materials.add(ColorMaterial::from_color(Color::srgb(0.3, 0.3, 0.3))),
        }
    }
}
