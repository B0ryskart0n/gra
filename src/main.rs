mod menu;
mod settings;
mod utils;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        //.add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin::default())
        //.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
        // Needs to be done after StatesPlugin (part of DefaultPlugins)
        .add_systems(Startup, startup)
        .add_plugins(menu::plugin)
        .add_plugins(settings::plugin)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((Camera2d,));
}
