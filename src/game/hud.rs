use bevy::prelude::*;

use super::components::*;
use super::resources::*;
use crate::GameState;

pub fn spawn(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                // SpaceBetween to place two children on top and bottom.
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..Node::DEFAULT
            },
            StateScoped(GameState::Game),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    ..Default::default()
                },
                EquipmentNode,
            ));
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((Text::default(), HealthHud));
                });
        });
}

pub fn update_health(
    mut q_health_hud: Query<&mut Text, With<HealthHud>>,
    q_health: Query<&Health, With<Player>>,
) -> Result {
    // TODO Doesn't this heap allocate new string with each Update?
    q_health_hud.single_mut()?.0 = q_health.single()?.0.to_string();
    Ok(())
}

// TODO Optimise this to not re-spawn items
pub fn update_equipment(
    mut commands: Commands,
    q_eq_node: Query<Entity, With<EquipmentNode>>,
    asset_server: Res<AssetServer>,
    equipment: Res<PlayerEquipment>,
) -> Result {
    commands
        .entity(q_eq_node.single()?)
        .despawn_related::<Children>()
        .with_children(|parent| equipment.hud_nodes(asset_server, parent));
    Ok(())
}
