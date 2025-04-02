#![allow(dead_code)]

use crate::GameState;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<UserSettings>()
        .add_systems(OnEnter(GameState::Settings), enter_settings)
        .add_systems(Update, check_settings_exit.run_if(in_state(GameState::Settings)))
        .add_systems(OnExit(GameState::Settings), exit_settings);
}

fn enter_settings() {}
fn check_settings_exit(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}
fn exit_settings() {}

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
