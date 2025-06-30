use bevy::prelude::*;

use super::*;
use crate::MainState;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    parent
                        .spawn((Node::default(), Skills))
                        .with_children(|parent| {
                            // TODO Generalise depending on character. Maybe use spritesheet?
                            parent
                                .spawn((ImageNode::from(asset_server.load("skill_a.png")), SkillA));
                            parent
                                .spawn((ImageNode::from(asset_server.load("skill_b.png")), SkillB));
                            parent
                                .spawn((ImageNode::from(asset_server.load("skill_c.png")), SkillC));
                            parent
                                .spawn((ImageNode::from(asset_server.load("skill_d.png")), SkillD));
                        });
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
    q_text.single_mut()?.0 = format!("{:.2}", stopwatch.0.elapsed_secs());
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

#[derive(Component)]
struct Skills;
#[derive(Component)]
struct SkillA;
#[derive(Component)]
struct SkillB;
#[derive(Component)]
struct SkillC;
#[derive(Component)]
struct SkillD;

#[derive(Component)]
pub struct HealthHud;
#[derive(Component)]
pub struct RunTime;
#[derive(Component)]
pub struct EquipmentNode;
