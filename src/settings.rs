// TODO Use Bevy settings

use bevy::prelude::*;
use bevy::window::WindowMode;

const LOGICAL_WIDTH: u32 = 640;
const LOGICAL_HEIGHT: u32 = 360;
const HD_WIDTH: u32 = 1280;
const HD_HEIGHT: u32 = 720;
const FULLHD_WIDTH: u32 = 1920;
const FULLHD_HEIGHT: u32 = 1080;
const QHD_WIDTH: u32 = 2560;
const QHD_HEIGHT: u32 = 1440;

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
    Borderless,
}
impl Default for WindowSettings {
    fn default() -> Self {
        Self::Windowed(Resolution::default())
    }
}
impl WindowSettings {
    pub fn apply_window_settings(&self, bevy_window: &mut Window) {
        match self {
            Self::Windowed(resolution) => {
                bevy_window.mode = WindowMode::Windowed;
                bevy_window
                    .resolution
                    .set_physical_resolution(resolution.x(), resolution.y());
                bevy_window.resolution.set_scale_factor(resolution.scale());
            }
            Self::Borderless => {
                bevy_window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
                // TODO Set resolution based on the physical resolution. Should create black bars when not scaled by integer
            }
        }
    }
    pub fn cycle_mode(&mut self) {
        *self = match self {
            Self::Windowed(_) => Self::Borderless,
            Self::Borderless => Self::default(),
        }
    }
    pub fn cycle_res(&mut self) {
        match self {
            Self::Windowed(res) => res.cycle(),
            _ => (),
        }
    }
    pub fn mode_str(&self) -> String {
        match self {
            Self::Windowed(_) => "Windowed".into(),
            Self::Borderless => "Borderless".into(),
        }
    }
    pub fn res_str(&self) -> String {
        match self {
            Self::Windowed(res) => res.to_string(),
            _ => "-".to_string(),
        }
    }
}

#[derive(Default, Debug)]
pub enum Resolution {
    Logical,
    #[default]
    HD,
    FullHD,
    QHD,
}
impl Resolution {
    fn scale(&self) -> f32 {
        match self {
            Self::Logical => 1.0,
            Self::HD => 2.0,
            Self::FullHD => 3.0,
            Self::QHD => 4.0,
        }
    }
    fn x(&self) -> u32 {
        match self {
            Self::Logical => LOGICAL_WIDTH,
            Self::HD => HD_WIDTH,
            Self::FullHD => FULLHD_WIDTH,
            Self::QHD => QHD_WIDTH,
        }
    }
    fn y(&self) -> u32 {
        match self {
            Self::Logical => LOGICAL_HEIGHT,
            Self::HD => HD_HEIGHT,
            Self::FullHD => FULLHD_HEIGHT,
            Self::QHD => QHD_HEIGHT,
        }
    }
    fn cycle(&mut self) {
        *self = match self {
            Self::Logical => Self::HD,
            Self::HD => Self::FullHD,
            Self::FullHD => Self::QHD,
            Self::QHD => Self::Logical,
        };
    }
}
impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} * {}", self.x(), self.y())
    }
}
