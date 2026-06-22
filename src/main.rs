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
                change_resolution.run_if(input_just_pressed(KeyCode::Space)),
                reset_resolution.run_if(input_just_pressed(KeyCode::Escape)),
            ),
        )
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(Text::default());
}
fn reset_resolution(mut q_window: Query<&mut Window>) {
    let mut bevy_window = q_window.single_mut().unwrap();
    bevy_window.resolution.set_scale_factor_override(None);
    bevy_window.resolution.set_physical_resolution(1280, 720);
}
fn change_resolution(mut q_window: Query<&mut Window>) {
    let mut bevy_window = q_window.single_mut().unwrap();
    bevy_window.resolution.set_scale_factor_override(Some(2.0));
    bevy_window.resolution.set_physical_resolution(1280, 720);
    // After setting the above I expect a window with HD size, but 640x360 internal resolution
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
