mod menu;
mod utils;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin::default())
        //.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
        .add_systems(Startup, startup)
        .add_plugins(menu::plugin)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((Camera2d,));
}
