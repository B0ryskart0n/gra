use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy::ui_widgets::Button;
use bevy::ui_widgets::observe;
use bevy::window::WindowMode;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin::default())
        //.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((Camera2d,));
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Node::DEFAULT
        })
        .with_children(|parent| {
            parent.spawn((
                Button,
                Text::new("Menu"),
                observe(|_: On<Activate>| info!("Menu")),
            ));
            parent.spawn((
                Button,
                Text::new("Apply"),
                observe(|_: On<Activate>, mut q_window: Query<&mut Window>| {
                    info!("Apply");
                    let mut bevy_window =
                        q_window.single_mut().expect("expected exactly one window");
                    bevy_window.mode = WindowMode::Windowed;
                    bevy_window.resolution.set_physical_resolution(1280, 720);
                    bevy_window.resolution.set_scale_factor(2.0);
                }),
            ));
        });
}
