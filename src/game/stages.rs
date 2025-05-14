use bevy::prelude::*;

pub fn stage0(mut commands: Commands) {
    commands.spawn(Stage(0));
}

#[derive(Component)]
#[allow(dead_code)]
struct Stage(u8);
