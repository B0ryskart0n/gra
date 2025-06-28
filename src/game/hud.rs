use bevy::prelude::*;

use super::*;
use crate::MainState;

#[derive(Component)]
pub struct HealthHud;

#[derive(Component)]
pub struct RunTime;

#[derive(Component)]
pub struct EquipmentNode;

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
            StateScoped(MainState::Game),
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((Node::default(), EquipmentNode));
                    parent.spawn((Text::default(), RunTime));
                });
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((Text::default(), HealthHud));
                });
        });
}

pub fn update_health(
    mut q_health_hud: Query<&mut Text, With<HealthHud>>,
    q_player: Query<(&Health, &Stats), With<Player>>,
) -> Result {
    let (health, stats) = q_player.single()?;
    q_health_hud.single_mut()?.0 = format!("{}/{}", health.0, stats.max_health);
    Ok(())
}

pub fn update_run_time(mut q_text: Query<&mut Text, With<RunTime>>, q_run: Query<&Run>) -> Result {
    let stopwatch = q_run.single()?;
    q_text.single_mut()?.0 = format!("{:?}", stopwatch.0.elapsed());
    Ok(())
}

/// Should only be run if `Equipment` changes, since it modifies components
pub fn update_equipment(
    mut commands: Commands,
    q_equipment_node: Query<Entity, With<EquipmentNode>>,
    q_equipment: Query<&Equipment>,
    asset_server: Res<AssetServer>,
) -> Result {
    let equipment = q_equipment.single()?;
    commands
        .entity(q_equipment_node.single()?)
        .despawn_related::<Children>()
        .with_children(|parent| equipment.hud_nodes(asset_server, parent));
    Ok(())
}
