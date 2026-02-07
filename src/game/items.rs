use super::*;
use bevy::prelude::*;
use std::cmp::Ordering;

const INTERACTION_DISTANCE: f32 = 1.0;

// TODO In case of many interactables later in the game development pressing E might cause stagger,
// which might be suboptimal. Consider not analysing all interactables when button pressed, but rather
// periodically (doesn't have to be each frame) keep track of all entities that are potentially
// to be interacted with and pressing E will just use that list.
pub fn pickup(
    mut commands: Commands,
    mut q_player: Query<(&GlobalTransform, &mut Equipment), With<Player>>,
    q_items: Query<(Entity, &Item, &GlobalTransform), Without<Player>>,
    mut pickup_messages: MessageWriter<ItemPickup>,
) -> Result {
    let (player_pos, mut equipment) = q_player.single_mut()?;
    // Finds the closest item within the `INTERACTION_DISTANCE` and picks it up.
    q_items
        .iter()
        .map(|(e, item, pos)| {
            (
                e,
                item,
                player_pos.translation().distance(pos.translation()),
            )
        })
        .filter(|(_, _, distance)| *distance < INTERACTION_DISTANCE)
        .min_by(|(_, _, x), (_, _, y)| x.partial_cmp(y).unwrap_or(Ordering::Equal))
        .map(|(entity, item, _)| {
            equipment.pickup(item.clone());
            commands.entity(entity).despawn();
            pickup_messages.write_default();
        });

    Ok(())
}
