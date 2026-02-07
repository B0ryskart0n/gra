mod enemy;
mod hud;
mod items;
mod pause;
mod player;
mod stages;

use crate::Cursor;
use crate::METERS_PER_PIXEL;
use crate::MainState;
use crate::PIXELS_PER_METER;
use crate::utils::Lifetime;
use avian2d::prelude::*;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use std::collections::HashMap;

const SPRITE_ORIENTATION: Vec2 = Vec2::Y;
const ENEMY_SPAWN_INTERVAL: f32 = 5.0;
/// Rate of exponential decay in the distance between camera and its goal.
const CAMERA_SPEED: f32 = 8.0;
const CURSOR_CAMERA_INFLUENCE: f32 = 0.3;
const ATTACK_SPEED: f32 = 2.0;
const PLAYER_SPEED: f32 = 3.0;
const PLAYER_MAX_HEALTH: f32 = 100.0;

pub fn game_plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default().with_length_unit(PIXELS_PER_METER))
        .insert_resource(Gravity(-10.0 * Vec2::Y))
        .add_sub_state::<GameSubState>()
        .add_message::<PlayerDeath>()
        .add_message::<PlayerDamage>()
        .add_message::<ItemPickup>()
        .add_message::<ChangeStage>()
        .clear_messages_on_exit::<PlayerDeath>(MainState::Game)
        .clear_messages_on_exit::<PlayerDamage>(MainState::Game)
        .clear_messages_on_exit::<ItemPickup>(MainState::Game)
        .clear_messages_on_exit::<ChangeStage>(MainState::Game)
        .add_systems(
            OnEnter(MainState::Game),
            (
                run_start,
                stages::stage0,
                player::spawn,
                hud::spawn,
                pause::spawn_invisible_overlay,
            ),
        )
        // Only game state changes the camera position so resetting camera when exiting it
        // should solve all with camera position.
        .add_systems(OnExit(MainState::Game), reset_camera)
        .add_systems(
            RunFixedMainLoop,
            update_camera_and_cursor
                .run_if(in_state(MainState::Game))
                .in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
        )
        .add_systems(
            FixedUpdate,
            (
                // TODO Consider running collision effects run after the physics
                player::hit,
                player::take_damage,
                player::attack,
                player::handle_input,
                enemy::hit,
                enemy::handle_state,
                Health::system,
                Lifetime::system,
            )
                .run_if(in_state(MainState::Game)),
        )
        .add_systems(
            Update,
            (
                enemy::spawn,
                stages::stage1.run_if(on_message::<ChangeStage>),
                pause::toggle.run_if(input_just_pressed(KeyCode::Escape)),
                player::visual_state,
                update_run,
                exit_game.run_if(input_just_pressed(KeyCode::F4).or(on_message::<PlayerDeath>)),
                player::update_stats.run_if(on_message::<ItemPickup>),
                stages::door_interaction.run_if(input_just_pressed(KeyCode::KeyE)),
                items::pickup.run_if(input_just_pressed(KeyCode::KeyE)),
                hud::update_run_time,
                hud::update_health,
                hud::update_equipment.run_if(on_message::<ItemPickup>),
            )
                .run_if(in_state(MainState::Game)),
        );
}
fn run_start(mut commands: Commands) {
    commands.spawn((
        Name::new("Run"),
        Run::default(),
        DespawnOnExit(MainState::Game),
    ));
}
fn update_run(time: Res<Time>, mut q_run: Query<&mut Run>) -> Result {
    q_run.single_mut()?.0.tick(time.delta());
    Ok(())
}
fn exit_game(mut next_state: ResMut<NextState<MainState>>) {
    next_state.set(MainState::Menu);
}
fn reset_camera(mut q_camera: Query<&mut Transform, With<Camera>>) -> Result {
    q_camera.single_mut()?.translation = Vec3::ZERO;
    Ok(())
}
fn update_camera_and_cursor(
    time: Res<Time>,
    mut q_camera: Query<(&Camera, &mut Transform, &GlobalTransform, &mut Cursor)>,
    // the Without<Camera> is required because both query Transform
    q_player: Query<&Transform, (With<Player>, Without<Camera>)>,
    q_window: Query<&Window>,
) -> Result {
    let (camera, mut camera_transform, camera_global_transform, mut cursor) =
        q_camera.single_mut()?;
    let player = q_player.single()?;
    let window = q_window.single()?;

    cursor.0 = window
        .cursor_position()
        .map(|viewport_position| {
            camera.viewport_to_world_2d(camera_global_transform, viewport_position)
        })
        .and_then(|res| res.ok());

    let camera_goal = match cursor.0 {
        // in case of no cursor on the screen just follow the player
        None => player.translation,
        Some(cursor_position) => {
            // calculate vector from camera to cursor and add that to player
            let direction = cursor_position.extend(0.0) - camera_global_transform.translation();
            player.translation + CURSOR_CAMERA_INFLUENCE * direction
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

#[derive(Message, Default)]
struct ItemPickup;
#[derive(Message, Default)]
struct PlayerDeath;
#[derive(Message)]
struct PlayerDamage(f32);
#[allow(dead_code)]
#[derive(Message)]
struct ChangeStage(u8);

#[derive(Component)]
struct EnemySpawner(Timer);
impl Default for EnemySpawner {
    fn default() -> Self {
        EnemySpawner(Timer::from_seconds(
            ENEMY_SPAWN_INTERVAL,
            TimerMode::Repeating,
        ))
    }
}
#[derive(Component)]
struct Health(f32);
impl Health {
    /// Despawns entities (except Player) with non-positive health.
    pub fn system(mut commands: Commands, query: Query<(Entity, &Health), Without<Player>>) {
        query.iter().for_each(|(e, h)| {
            if h.0 <= 0.0 {
                commands.entity(e).despawn();
            }
        })
    }
}
#[derive(Component)]
struct Projectile;
#[derive(Component)]
struct Player;
#[derive(Component)]
struct Enemy;
#[derive(Component)]
struct Stats {
    max_health: f32,
    attack_speed: f32,
    _movement_speed: f32,
}
impl Default for Stats {
    fn default() -> Self {
        Self {
            max_health: PLAYER_MAX_HEALTH,
            attack_speed: ATTACK_SPEED,
            _movement_speed: PLAYER_SPEED,
        }
    }
}
impl Stats {
    fn apply_equipment(&mut self, eq: &Equipment) {
        self.attack_speed *= 1.0 + eq.item_stat(&Item::Banana);
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
    // FIXME Knowing the pixel (and meter) size should not be runtime.
    fn size(&self) -> Vec2 {
        match self {
            Self::Banana => METERS_PER_PIXEL * Vec2::new(16.0, 16.0),
        }
    }
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
struct Run(Stopwatch);

#[derive(Default, PhysicsLayer)]
enum CollisionGroup {
    #[default]
    Default,
    Terrain,
    Player,
    Projectile,
    Enemy,
}
