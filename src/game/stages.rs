use super::*;
use bevy::prelude::*;

pub fn stage0(q_stages: Query<Entity, With<Stage>>, mut commands: Commands) {
    // Make sure that there is one stage at a time.
    q_stages.iter().for_each(|stage| {
        commands.entity(stage).despawn();
    });
    // TODO Fix this door not being visible
    commands
        .spawn((
            Stage,
            Sprite::from_color(Color::srgb(0.4, 0.4, 0.4), Vec2::splat(200.0)),
            DespawnOnExit(MainState::Game),
        ))
        .with_child((Door(1), Sprite::from_color(Color::BLACK, Vec2::splat(20.0))));
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
            Stage,
            Sprite::from_color(Color::srgb(0.4, 0.4, 0.4), Vec2::from((400.0, 300.0))),
            EnemySpawner::default(),
            DespawnOnExit(MainState::Game),
        ))
        .with_children(|parent| {
            parent.spawn((
                Item::Banana,
                Sprite::from_image(Item::Banana.image(&asset_server)),
                Transform::from_translation(Vec3::from((100.0, -100.0, 0.4))),
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
        if player_pos.translation().distance(door_pos.translation()) <= 10.0 {
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
