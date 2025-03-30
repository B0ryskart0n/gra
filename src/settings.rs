#![allow(dead_code)]

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<UserSettings>();
}

#[derive(Resource, Default)]
pub struct UserSettings {
    pub resolution: Resolution,
}

#[derive(Default)]
pub enum Resolution {
    FullHD,
    #[default]
    HD,
    Custom([i32; 2]),
}
impl Resolution {
    fn pixels(self) -> [i32; 2] {
        match self {
            Self::FullHD => [1920, 1080],
            Self::HD => [1280, 720],
            Self::Custom(x) => x,
        }
    }
}
