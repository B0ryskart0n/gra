use std::collections::HashMap;

use bevy::color::palettes::css::{BLUE, GREY, RED};
use bevy::prelude;

struct ColorMaterialHandles(HashMap<String, Handle<ColorMaterial>>);

// TODO init with basic color materials
