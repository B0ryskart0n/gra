use crate::utils;

use super::*;
use bevy::prelude::*;

pub fn stage0(q_stages: Query<Entity, With<Stage>>, mut commands: Commands) {
    // Make sure that there is one stage at a time.
    q_stages.iter().for_each(|stage| {
        commands.entity(stage).despawn();
    });
    commands
        .spawn((
            Name::new("Stage 0"),
            Stage,
            RigidBody::Static,
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            Sprite::from_color(Color::WHITE, Vec2::new(30.0, 1.0)),
            Collider::rectangle(30.0, 1.0),
            CollisionLayers::new(CollisionGroup::Terrain, LayerMask::ALL),
            DespawnOnExit(MainState::Game),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::from("Door 1"),
                Door(1),
                Sprite::from_color(Color::BLACK, Vec2::splat(2.0)),
                Transform::from_translation(Vec3::new(5.0, 1.5, 0.2)),
            ));
        });
}

pub fn stage1(
    q_stages: Query<Entity, With<Stage>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Make sure that there is one stage at a time.
    q_stages.iter().for_each(|stage| {
        commands.entity(stage).despawn();
    });
    commands
        .spawn((
            Name::new("Stage 1"),
            Stage,
            RigidBody::Static,
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            Sprite::from_color(Color::WHITE, Vec2::new(100.0, 1.0)),
            Collider::rectangle(100.0, 1.0),
            CollisionLayers::new(CollisionGroup::Terrain, LayerMask::ALL),
            DespawnOnExit(MainState::Game),
        ))
        .with_children(|parent| {
            parent.spawn((
                Transform::from_translation(Vec3::new(18.0, 9.0, 0.5)),
                EnemySpawner::default(),
            ));
            parent.spawn((
                Item::Banana,
                utils::image_size_to_sprite(Item::Banana.image(&asset_server), Item::Banana.size()),
                RigidBody::Dynamic,
                Collider::rectangle(Item::Banana.size().x, Item::Banana.size().y),
                CollisionLayers::new(CollisionGroup::Default, CollisionGroup::Terrain),
                Mass(100.0),
                Transform::from_translation(Vec3::new(-3.0, 5.0, 0.4)),
            ));
        });
}
// TODO Maybe create common logic for general interaction, regardless of door/item
pub fn door_interaction(
    q_door: Query<(&GlobalTransform, &Door)>,
    q_player: Query<&GlobalTransform, With<Player>>,
    mut change_stage_messages: MessageWriter<ChangeStage>,
) -> Result {
    let player_pos = q_player.single()?;

    // Copied from item pickup function just to solve the issue of clicking E with no Doors.
    q_door.iter().for_each(|(door_pos, door)| {
        if player_pos.translation().distance(door_pos.translation()) <= 1.0 {
            change_stage_messages.write(ChangeStage(door.0));
        }
    });

    Ok(())
}

#[derive(Component)]
pub struct Stage;

/// Door to a specific stage.
#[derive(Component)]
pub struct Door(u8);
