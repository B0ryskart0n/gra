mod enemy;
mod hud;
mod items;
mod pause;
mod player;
mod stages;

use super::CursorPosition;
use super::MainState;
use super::utils;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use std::collections::HashMap;
use std::mem::discriminant;

const DASH_TIME: f32 = 0.4;
const ENEMY_SPAWN_INTERVAL: f32 = 5.0;
const ENEMY_SIZE: f32 = 15.0;
const ENEMY_HEALTH: f32 = 3.0;
const ENEMY_SPEED: f32 = 100.0;
/// Rate of exponential decay in the distance between camera and it's goal.
const CAMERA_SPEED: f32 = 8.0;
const CURSOR_CAMERA_INFLUENCE: f32 = 0.3;
const ATTACK_SPEED: f32 = 2.0;
const PLAYER_SIZE: f32 = 25.0;
const PLAYER_SPEED: f32 = 120.0;
const PLAYER_HEALTH: f32 = 100.0;

pub fn game_plugin(app: &mut App) {
    app.add_sub_state::<GameSubState>()
        // Since I want to rely on `resource_changed` condition I need to initiate
        // those resources at the top level instead of `OnEnter(GameState::Game)`.
        .init_resource::<EnemySpawn>()
        .add_state_scoped_event::<PlayerDeath>(MainState::Game)
        .add_state_scoped_event::<ItemPickup>(MainState::Game)
        .add_systems(
            OnEnter(MainState::Game),
            (
                utils::reset_camera,
                stages::stage0,
                items::spawn,
                player::spawn,
                hud::spawn,
                pause::spawn_invisible_overlay,
            ),
        )
        .add_systems(
            RunFixedMainLoop,
            player::handle_input
                .run_if(in_state(MainState::Game))
                .in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
        )
        .add_systems(
            FixedUpdate,
            (
                enemy::spawn,
                player::hit,
                (enemy::hit, enemy::despawn_unhealthy).chain(),
                player::attack,
                utils::lifetime,
                (player::handle_state, enemy::handle_state, physics).chain(),
            )
                .run_if(in_state(MainState::Game)),
        )
        .add_systems(
            Update,
            (
                pause::toggle.run_if(input_just_pressed(KeyCode::Escape)),
                player::visual_state,
                update_camera,
                exit_game.run_if(input_just_pressed(KeyCode::F4).or(on_event::<PlayerDeath>)),
                player::update_stats.run_if(on_event::<ItemPickup>),
                items::pickup.run_if(input_just_pressed(KeyCode::KeyE)),
                hud::update_health,
                hud::update_equipment.run_if(on_event::<ItemPickup>),
            )
                .run_if(in_state(MainState::Game)),
        );
}

fn exit_game(mut next_state: ResMut<NextState<MainState>>) {
    next_state.set(MainState::Menu);
}
fn physics(time_fixed: Res<Time<Fixed>>, mut query: Query<(&mut Transform, &Velocity)>) {
    query
        .iter_mut()
        .for_each(|(mut transform, vel)| transform.translation += vel.0 * time_fixed.delta_secs());
}
fn update_camera(
    time: Res<Time>,
    cursor_position: Res<CursorPosition>,
    mut camera_query: Query<(&mut Transform, &GlobalTransform), With<Camera2d>>,
    // the Without<Camera2d> is required because both query Transform
    player_transform: Single<&Transform, (With<Player>, Without<Camera2d>)>,
) -> Result {
    let (mut camera_transform, camera_global_transform) = camera_query.single_mut()?;

    let camera_goal = match cursor_position.0 {
        // in case of no cursor on the screen just follow the player
        None => player_transform.translation,
        Some(cursor_position) => {
            // calculate vector from camera to cursor and add that to player
            let direction = cursor_position.extend(0.0) - camera_global_transform.translation();
            player_transform.translation + CURSOR_CAMERA_INFLUENCE * direction
        }
    }
    .with_z(camera_global_transform.translation().z);

    camera_transform
        .translation
        .smooth_nudge(&camera_goal, CAMERA_SPEED, time.delta_secs());

    Ok(())
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, SubStates)]
#[source(MainState = MainState::Game)]
enum GameSubState {
    #[default]
    Running,
    Paused,
}

#[derive(Event, Default)]
struct ItemPickup;

#[derive(Event, Default)]
struct PlayerDeath;

#[derive(Resource)]
struct EnemySpawn(Timer);
impl Default for EnemySpawn {
    fn default() -> Self {
        EnemySpawn(Timer::from_seconds(
            ENEMY_SPAWN_INTERVAL,
            TimerMode::Repeating,
        ))
    }
}

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Health(f32);

#[derive(Component)]
struct Projectile;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component, PartialEq, Default)]
enum PlayerState {
    #[default]
    Idle,
    Dashing(Vec3),
    Attacking,
}
impl PlayerState {
    fn is_dashing(&self) -> bool {
        discriminant(self) == discriminant(&PlayerState::Dashing(Vec3::ZERO))
    }
}

#[derive(Component)]
struct Stats {
    max_health: f32,
    attack_speed: f32,
    movement_speed: f32,
}
impl Default for Stats {
    fn default() -> Self {
        Self {
            max_health: PLAYER_HEALTH,
            attack_speed: ATTACK_SPEED,
            movement_speed: PLAYER_SPEED,
        }
    }
}
impl Stats {
    fn apply_equipment(&mut self, eq: &Equipment) {
        self.attack_speed *= 1.0 + eq.item_stat(&Item::Banana);
    }
}

#[derive(Component)]
struct DashTimer(Timer);
impl Default for DashTimer {
    fn default() -> Self {
        DashTimer(Timer::from_seconds(DASH_TIME, TimerMode::Once))
    }
}

#[derive(Component, Default)]
struct Equipment(HashMap<Item, u8>);
impl Equipment {
    fn pickup(&mut self, item: Item) {
        self.0
            .entry(item)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    fn hud_nodes(&self, asset_server: Res<AssetServer>, spawner: &mut ChildSpawnerCommands) {
        self.0
            .iter()
            .filter(|(_, val)| **val != 0u8)
            .for_each(|(key, _)| {
                spawner.spawn(ImageNode::new(key.image(&asset_server)));
            });
    }
    fn item_stat(&self, item: &Item) -> f32 {
        *self.0.get(item).unwrap_or(&0u8) as f32 * Item::stat(item)
    }
}

#[derive(Component, PartialEq, Eq, Hash, Clone)]
enum Item {
    Banana,
}
impl Item {
    fn image(&self, asset_server: &Res<AssetServer>) -> Handle<Image> {
        match self {
            Self::Banana => asset_server.load("banana.png"),
        }
    }
    fn stat(&self) -> f32 {
        match self {
            // Attack speed
            Self::Banana => 0.5,
        }
    }
}

#[derive(Component, Default)]
struct PlayerInput {
    direction: Vec3,
    dash: bool,
    attack: bool,
}

/// Timer of 1 second, scaled by the `Stats` `attack_speed`
#[derive(Component, Deref, DerefMut)]
struct AttackTimer(Timer);
impl Default for AttackTimer {
    fn default() -> Self {
        AttackTimer(Timer::from_seconds(1.0, TimerMode::Once))
    }
}
