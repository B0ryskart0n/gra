use super::*;
use bevy::prelude::*;

pub fn stage0(mut commands: Commands) {
    commands.spawn((
        Stage(0),
        Sprite::from_color(Color::srgb(0.4, 0.4, 0.4), Vec2::from((200.0, 200.0))),
        StateScoped(MainState::Game),
    ));
}

pub fn stage1(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Stage(1),
            Sprite::from_color(Color::srgb(0.4, 0.4, 0.4), Vec2::from((400.0, 300.0))),
            EnemySpawner::default(),
            StateScoped(MainState::Game),
        ))
        .with_children(|parent| {
            parent.spawn((
                Item::Banana,
                Sprite::from_image(asset_server.load("banana.png")),
                Transform::from_translation(Vec3::from((100.0, -100.0, 0.4))),
            ));
        });
}

#[derive(Component)]
#[allow(dead_code)]
struct Stage(u8);
