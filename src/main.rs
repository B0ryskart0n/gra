use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                update_resolution_text,
                reset_window.run_if(input_just_pressed(KeyCode::Digit0)),
                change_base_scale.run_if(input_just_pressed(KeyCode::Digit1)),
                change_scale_override.run_if(input_just_pressed(KeyCode::Digit2)),
            ),
        )
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(Text::default());
    // Visual check of the actual logical resolution.
    // In 640x360 this text should be at the lower boundary of the window, barely visible.
    commands.spawn((
        Text2d::new("I'm at [0, -180]"),
        Transform::from_translation(vec3(0.0, -180.0, 0.0)),
    ));
}
fn reset_window(mut q_window: Query<&mut Window>) {
    let mut bevy_window = q_window.single_mut().unwrap();
    bevy_window.resolution.set_scale_factor(1.0);
    bevy_window.resolution.set_scale_factor_override(None);
    bevy_window.resolution.set_physical_resolution(1280, 720);
}
fn change_base_scale(mut q_window: Query<&mut Window>) {
    let mut bevy_window = q_window.single_mut().unwrap();
    bevy_window.resolution.set_scale_factor(2.0);
    bevy_window.resolution.set_physical_resolution(1280, 720);
}
fn change_scale_override(mut q_window: Query<&mut Window>) {
    let mut bevy_window = q_window.single_mut().unwrap();
    bevy_window.resolution.set_scale_factor_override(Some(2.0));
    bevy_window.resolution.set_physical_resolution(1280, 720);
}
/// Only for informative purposes, to display the current window state.
fn update_resolution_text(mut q_text: Query<&mut Text>, q_window: Query<&Window>) {
    let res = q_window.single().unwrap().resolution.clone();

    q_text.single_mut().unwrap().0 = format!(
        "Logical size: {}\nPhysical size: {}\nBase scale: {}\nScale override: {:?}",
        res.size(),
        res.physical_size(),
        res.base_scale_factor(),
        res.scale_factor_override()
    );
}
