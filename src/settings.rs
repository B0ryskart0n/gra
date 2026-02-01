use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::window::WindowResolution;

const LOGICAL_WIDTH: u32 = 640;
const LOGICAL_HEIGHT: u32 = 360;
const HD_WIDTH: u32 = 1280;
const HD_HEIGHT: u32 = 720;
const FULLHD_WIDTH: u32 = 1920;
const FULLHD_HEIGHT: u32 = 1080;
const QHD_WIDTH: u32 = 2560;
const QHD_HEIGHT: u32 = 1440;

pub fn plugin(app: &mut App) {
    app.init_resource::<UserSettings>()
        .add_systems(PreStartup, startup_window_settings);
}

fn startup_window_settings(
    mut q_window: Query<&mut Window>,
    settings: Res<UserSettings>,
) -> Result {
    let mut window = q_window.single_mut()?;
    settings.window.set_bevy(&mut window);
    Ok(())
}

// TODO Add loading last settings and falling back to creating defaults from system settings.
// TODO Create a mechanism for synchronizing with bevy settings to avoid desynchronization.
// As those settings grow, it might be good to do a round trip. Something like:
// `bevy_window = user_window.to_bevy(); user_window = bevy_window.to_user();` or maybe use the Reflect trait?
#[derive(Debug, Resource, Default)]
pub struct UserSettings {
    pub window: WindowSettings,
}
// TODO Take Monitor information into account when determining resoltuions.
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
    pub fn set_bevy(&self, bevy_window: &mut Window) {
        // Setting the mode before the resolution seems to work better.
        bevy_window.position = WindowPosition::Centered(MonitorSelection::Current);
        bevy_window.mode = self.to_bevy_mode();
        bevy_window.resolution = self.to_bevy_res();
    }
    fn to_bevy_mode(&self) -> WindowMode {
        match self {
            Self::Windowed(_) => WindowMode::Windowed,
            Self::Borderless => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
        };

        // No need to touch the resolution if not in Windowed.
        if let Self::Windowed(resolution) = self {
            bevy_window.resolution = WindowResolution::from(resolution.pixels())
                .with_scale_factor_override(resolution.scale());
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
    #[default]
    Logical,
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
    fn pixels(&self) -> [u32; 2] {
        match self {
            Self::Logical => [LOGICAL_WIDTH, LOGICAL_HEIGHT],
            Self::HD => [HD_WIDTH, HD_HEIGHT],
            Self::FullHD => [FULLHD_WIDTH, FULLHD_HEIGHT],
            Self::QHD => [QHD_WIDTH, QHD_HEIGHT],
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
        write!(f, "{} * {}", self.pixels()[0], self.pixels()[1])
    }
}
