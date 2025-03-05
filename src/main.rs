use bevy::prelude::*;

mod game;
mod menu;
// mod settings;
mod splash;
mod utils;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    //Settings,
    Game,
    //Pause
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>() // Initial state will be the #[default]
        .enable_state_scoped_entities::<GameState>()
        .add_systems(Startup, spawn_camera)
        .add_plugins(splash::splash_plugin)
        .add_plugins(menu::menu_plugin)
        .add_plugins(game::game_plugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
