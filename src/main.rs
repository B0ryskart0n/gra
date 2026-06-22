use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy::ui_widgets::Button;
use bevy::ui_widgets::observe;
use bevy::window::WindowResolution;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(640, 360).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GlobalUiDebugOptions {
            enabled: true,
            ..default()
        })
        .add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin::default())
        .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
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
        .with_child((
            Button,
            Text::new("Apply"),
            observe(|_: On<Activate>, mut q_window: Query<&mut Window>| {
                info!("Apply");
                let mut bevy_window = q_window.single_mut().expect("expected exactly one window");

                // Hyprland has a bug and only when window is maximized it will take resize requests
                // So debugging this issue is problematic on Hyprland
                bevy_window.set_maximized(true);

                info!("res_before: {:?}", bevy_window.resolution.clone());

                bevy_window.resolution.set_scale_factor(0.008);
                bevy_window.resolution.set_scale_factor_override(Some(3.0));
                bevy_window.resolution.set_physical_resolution(1280, 360);

                info!("res_after: {:?}", bevy_window.resolution);
            }),
        ));
}
