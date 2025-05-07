use super::components::*;
use crate::CursorPosition;
use crate::MainState;
use crate::utils::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::f32::consts::FRAC_1_SQRT_2;
use std::time::Duration;

const PROJECTILE_SIZE: f32 = 2.0;
const PROJECTILE_LIFETIME: f32 = 1.0;
const PROJECTILE_SPEED: f32 = 400.0;
const DASH_TIME: f32 = 0.4;
const DIRECTION_RIGHT: Vec3 = Vec3::X;
const DIRECTION_UPRIGHT: Vec3 = Vec3 {
    x: FRAC_1_SQRT_2,
    y: FRAC_1_SQRT_2,
    z: 0.0,
};
const DIRECTION_UP: Vec3 = Vec3::Y;
const DIRECTION_UPLEFT: Vec3 = Vec3 {
    x: -FRAC_1_SQRT_2,
    y: FRAC_1_SQRT_2,
    z: 0.0,
};
const DIRECTION_LEFT: Vec3 = Vec3::NEG_X;
const DIRECTION_DOWNLEFT: Vec3 = Vec3 {
    x: -FRAC_1_SQRT_2,
    y: -FRAC_1_SQRT_2,
    z: 0.0,
};
const DIRECTION_DOWN: Vec3 = Vec3::NEG_Y;
const DIRECTION_DOWNRIGHT: Vec3 = Vec3 {
    x: FRAC_1_SQRT_2,
    y: -FRAC_1_SQRT_2,
    z: 0.0,
};
const ATTACK_SPEED: f32 = 2.0;
pub const PLAYER_SIZE: f32 = 25.0;
const PLAYER_SPEED: f32 = 120.0;
const PLAYER_HEALTH: f32 = 100.0;

pub fn spawn(mut commands: Commands) {
    commands.spawn((
        Player,
        Health(PLAYER_HEALTH),
        Stats::default(),
        Sprite::from_color(Color::WHITE, Vec2::from((PLAYER_SIZE, PLAYER_SIZE))),
        PlayerState::default(),
        Transform::from_translation(Vec3::from((0.0, 0.0, 1.0))),
        Velocity(Vec3::ZERO),
        StateScoped(MainState::Game),
    ));
}

#[derive(Resource)]
pub struct DashTimer(pub Timer);
impl Default for DashTimer {
    fn default() -> Self {
        DashTimer(Timer::from_seconds(DASH_TIME, TimerMode::Once))
    }
}

/// In case of high frame rate (bigger than `FixedTime` 64Hz), if one swift button press is registered and
/// that input is overriden in  next schedule run (when the button is already released) and
/// the `FixedUpdate` schedule did not run, because the two frames were too close to each other,
/// then the swift input is effectively discarded.
pub fn handle_input(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_input: ResMut<PlayerInput>,
) {
    let left = keyboard.pressed(KeyCode::KeyA);
    let right = keyboard.pressed(KeyCode::KeyD);
    let down = keyboard.pressed(KeyCode::KeyS);
    let up = keyboard.pressed(KeyCode::KeyW);

    player_input.direction = match (left, right, down, up) {
        (false, false, false, false)
        | (true, true, true, true)
        | (true, true, false, false)
        | (false, false, true, true) => Vec3::ZERO,
        (false, true, false, false) | (false, true, true, true) => DIRECTION_RIGHT,
        (false, true, false, true) => DIRECTION_UPRIGHT,
        (false, false, false, true) | (true, true, false, true) => DIRECTION_UP,
        (true, false, false, true) => DIRECTION_UPLEFT,
        (true, false, false, false) | (true, false, true, true) => DIRECTION_LEFT,
        (true, false, true, false) => DIRECTION_DOWNLEFT,
        (false, false, true, false) | (true, true, true, false) => DIRECTION_DOWN,
        (false, true, true, false) => DIRECTION_DOWNRIGHT,
    };

    player_input.dash = keyboard.any_just_pressed(vec![KeyCode::ShiftLeft, KeyCode::Space]);
    player_input.attack = mouse.pressed(MouseButton::Left);
}

pub fn visual_state(mut query: Query<(&mut Sprite, &PlayerState), Changed<PlayerState>>) {
    query.iter_mut().for_each(|(mut sprite, state)| match *state {
        PlayerState::Idle => sprite.color = Color::srgb(0.1, 1.0, 0.1),
        PlayerState::Attacking => sprite.color = Color::srgb(1.0, 0.1, 0.1),
        PlayerState::Dashing(_) => sprite.color = Color::srgb(0.1, 0.1, 1.0),
    })
}

pub fn handle_state(
    time_fixed: Res<Time<Fixed>>,
    mut query: Query<(&mut PlayerState, &mut Velocity, &Stats), With<Player>>,
    input: Res<PlayerInput>,
    mut dash_timer: ResMut<DashTimer>,
) -> Result {
    let (mut state, mut velocity, stats) = query.single_mut()?;

    let dt = time_fixed.delta();

    if state.is_dashing() && dash_timer.0.tick(dt).finished() {
        *state = PlayerState::Idle;
        dash_timer.0.reset();
    }
    if !state.is_dashing() {
        *state = match (input.dash, input.attack) {
            (true, _) => PlayerState::Dashing(input.direction),
            (false, true) => PlayerState::Attacking,
            (false, false) => PlayerState::Idle,
        };
    }

    let base_velocity = match *state {
        PlayerState::Idle => 1.0 * input.direction,
        PlayerState::Dashing(dash) => 2.5 * dash,
        PlayerState::Attacking => 0.5 * input.direction,
    };

    velocity.0 = base_velocity * stats.movement_speed;

    Ok(())
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Stats {
    pub max_health: f32,
    pub attack_speed: f32,
    pub movement_speed: f32,
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
    pub fn apply_equipment(&mut self, eq: &PlayerEquipment) {
        self.attack_speed *= 1.0 + eq.item_stat(&Item::Banana);
    }
}

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub direction: Vec3,
    pub dash: bool,
    pub attack: bool,
}

#[derive(PartialEq, Default, Component)]
pub enum PlayerState {
    #[default]
    Idle,
    Dashing(Vec3),
    Attacking,
}
impl PlayerState {
    pub fn is_dashing(&self) -> bool {
        match self {
            PlayerState::Dashing(_) => true,
            _ => false,
        }
    }
}

#[derive(Resource, Default)]
pub struct PlayerEquipment(HashMap<Item, u8>);
impl PlayerEquipment {
    pub fn pickup(&mut self, item: Item) {
        self.0.entry(item).and_modify(|count| *count += 1).or_insert(1);
    }
    pub fn hud_nodes(&self, asset_server: Res<AssetServer>, spawner: &mut ChildSpawnerCommands) {
        self.0.iter().filter(|(_, val)| **val != 0u8).for_each(|(key, _)| {
            spawner.spawn(ImageNode::new(key.image(&asset_server)));
        });
    }
    pub fn item_stat(&self, item: &Item) -> f32 {
        *self.0.get(item).unwrap_or(&0u8) as f32 * Item::stat(item)
    }
}

pub fn player_hit(
    q_enemies: Query<&GlobalTransform, With<Enemy>>,
    mut q_player: Query<(&mut Health, &GlobalTransform, &PlayerState), (With<Player>, Without<Enemy>)>,
    mut death_events: EventWriter<PlayerDeath>,
) -> Result {
    let (mut player_health, player_transform, player_state) = q_player.single_mut()?;
    let damage = match *player_state {
        PlayerState::Dashing(_) => 0.0,
        _ => q_enemies
            .iter()
            .map(|enemy_transform| {
                square_collide(
                    player_transform.translation(),
                    PLAYER_SIZE,
                    enemy_transform.translation(),
                    super::ENEMY_SIZE,
                )
            })
            .filter(|hit| *hit)
            .count() as f32,
    };

    player_health.0 -= damage;
    if player_health.0 <= 0.0 {
        death_events.write_default();
    }

    Ok(())
}

pub fn attack(
    time_fixed: Res<Time<Fixed>>,
    mut commands: Commands,
    query: Query<(&GlobalTransform, &PlayerState, &Stats), With<Player>>,
    cursor_position: Res<CursorPosition>,
    mut attack_timer: ResMut<AttackTimer>,
) -> Result {
    let (player_transform, player_state, stats) = query.single()?;
    let player_position = player_transform.translation();

    attack_timer.tick(Duration::mul_f32(time_fixed.delta(), stats.attack_speed));

    if *player_state == PlayerState::Attacking && attack_timer.0.finished() {
        commands.spawn((
            Projectile,
            Sprite::from_color(Color::WHITE, Vec2::from((PROJECTILE_SIZE, PROJECTILE_SIZE))),
            Transform::from_translation(player_position),
            Velocity(Vec3::lerp(
                Vec3::ZERO,
                (cursor_position.0.unwrap_or(Vec2::X).extend(0.0) - player_position).normalize(),
                PROJECTILE_SPEED,
            )),
            StateScoped(MainState::Game),
            Lifetime(Timer::from_seconds(PROJECTILE_LIFETIME, TimerMode::Once)),
        ));
        attack_timer.0.reset();
    }

    Ok(())
}

/// Timer of 1 second, scaled by the `Stats` `attack_speed`
#[derive(Resource, Deref, DerefMut)]
pub struct AttackTimer(pub Timer);
impl Default for AttackTimer {
    fn default() -> Self {
        AttackTimer(Timer::from_seconds(1.0, TimerMode::Once))
    }
}

#[derive(Event, Default)]
pub struct PlayerDeath;
