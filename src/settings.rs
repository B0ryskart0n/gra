use bevy::prelude::*;
use bevy::window::WindowMode;

const HD_WIDTH: u32 = 1280;
const HD_HEIGHT: u32 = 720;

pub fn plugin(app: &mut App) {
    app.init_resource::<UserSettings>();
}

#[derive(Debug, Resource, Default)]
pub struct UserSettings {
    pub window: WindowSettings,
}
impl UserSettings {
    pub fn apply_settings(&self, mut q_window: Query<&mut Window>) {
        let mut bevy_window = q_window.single_mut().expect("expected exactly one window");
        self.window.apply_window_settings(&mut bevy_window);
    }
}
#[derive(Debug)]
pub enum WindowSettings {
    Windowed(Resolution),
}
impl Default for WindowSettings {
    fn default() -> Self {
        Self::Windowed(Resolution::default())
    }
}
impl WindowSettings {
    pub fn apply_window_settings(&self, bevy_window: &mut Window) {
        bevy_window.mode = WindowMode::Windowed;
        bevy_window
            .resolution
            .set_physical_resolution(HD_WIDTH, HD_HEIGHT);
        bevy_window.resolution.set_scale_factor(2.0);
    }
}

#[derive(Default, Debug)]
pub enum Resolution {
    #[default]
    HD,
}
