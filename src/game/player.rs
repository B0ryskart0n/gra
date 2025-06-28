use super::*;
use crate::CursorPosition;
use crate::MainState;
use crate::utils::*;
use bevy::prelude::*;
use std::f32::consts::FRAC_1_SQRT_2;

const PROJECTILE_SIZE: f32 = 2.0;
const PROJECTILE_LIFETIME: f32 = 1.0;
const PROJECTILE_SPEED: f32 = 400.0;
const DIRECTION_RIGHT: Vec2 = Vec2::X;
const DIRECTION_UPRIGHT: Vec2 = Vec2 {
    x: FRAC_1_SQRT_2,
    y: FRAC_1_SQRT_2,
};
const DIRECTION_UP: Vec2 = Vec2::Y;
const DIRECTION_UPLEFT: Vec2 = Vec2 {
    x: -FRAC_1_SQRT_2,
    y: FRAC_1_SQRT_2,
};
const DIRECTION_LEFT: Vec2 = Vec2::NEG_X;
const DIRECTION_DOWNLEFT: Vec2 = Vec2 {
    x: -FRAC_1_SQRT_2,
    y: -FRAC_1_SQRT_2,
};
const DIRECTION_DOWN: Vec2 = Vec2::NEG_Y;
const DIRECTION_DOWNRIGHT: Vec2 = Vec2 {
    x: FRAC_1_SQRT_2,
    y: -FRAC_1_SQRT_2,
};

pub fn spawn(mut commands: Commands) {
    commands.spawn((
        Player,
        DashTimer::default(),
        AttackTimer::default(),
        PlayerInput::default(),
        Health(PLAYER_HEALTH),
        Equipment::default(),
        Stats::default(),
        Sprite::from_color(Color::WHITE, Vec2::from((PLAYER_SIZE, PLAYER_SIZE))),
        PlayerState::default(),
        Transform::from_translation(Vec3::from((0.0, 0.0, 1.0))),
        Velocity(Vec2::ZERO),
        StateScoped(MainState::Game),
    ));
}
pub fn update_stats(mut q_player: Query<(&mut Stats, &Equipment)>) -> Result {
    let (mut stats, eq) = q_player.single_mut()?;
    stats.apply_equipment(&eq);
    Ok(())
}
/// In case of high frame rate (bigger than `FixedTime` 64Hz), if one swift button press is registered and
/// that input is overriden in  next schedule run (when the button is already released) and
/// the `FixedUpdate` schedule did not run, because the two frames were too close to each other,
/// then the swift input is effectively discarded.
pub fn handle_input(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut q_input: Query<&mut PlayerInput>,
) -> Result {
    let mut player_input = q_input.single_mut()?;

    let left = keyboard.pressed(KeyCode::KeyA);
    let right = keyboard.pressed(KeyCode::KeyD);
    let down = keyboard.pressed(KeyCode::KeyS);
    let up = keyboard.pressed(KeyCode::KeyW);

    player_input.direction = match (left, right, down, up) {
        (false, false, false, false)
        | (true, true, true, true)
        | (true, true, false, false)
        | (false, false, true, true) => Vec2::ZERO,
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
    Ok(())
}
pub fn visual_state(mut query: Query<(&mut Sprite, &PlayerState), Changed<PlayerState>>) {
    query
        .iter_mut()
        .for_each(|(mut sprite, state)| match *state {
            PlayerState::Idle => sprite.color = Color::srgb(0.1, 1.0, 0.1),
            PlayerState::Attacking => sprite.color = Color::srgb(1.0, 0.1, 0.1),
            PlayerState::Dashing(_) => sprite.color = Color::srgb(0.1, 0.1, 1.0),
        })
}
pub fn handle_state(
    time_fixed: Res<Time<Fixed>>,
    cursor_position: Res<CursorPosition>,
    mut q_player: Query<
        (
            &PlayerInput,
            &mut PlayerState,
            &mut Velocity,
            &mut DashTimer,
            &mut Transform,
            &Stats,
        ),
        With<Player>,
    >,
) -> Result {
    let (input, mut state, mut velocity, mut dash_timer, mut transform, stats) =
        q_player.single_mut()?;

    if let Some(cursor) = cursor_position.0 {
        let cursor_direction = (transform.translation.xy() - cursor).normalize();
        transform.rotation = Quat::from_rotation_arc_2d(SPRITE_ORIENTATION, cursor_direction);
    }

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
pub fn hit(
    q_enemies: Query<&GlobalTransform, With<Enemy>>,
    mut q_player: Query<
        (&mut Health, &GlobalTransform, &PlayerState),
        (With<Player>, Without<Enemy>),
    >,
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
    mut q_player: Query<(&mut AttackTimer, &GlobalTransform, &PlayerState, &Stats), With<Player>>,
    cursor_position: Res<CursorPosition>,
) -> Result {
    let (mut attack_timer, player_transform, player_state, stats) = q_player.single_mut()?;
    let player_position = player_transform.translation();

    attack_timer.tick(time_fixed.delta().mul_f32(stats.attack_speed));

    if *player_state == PlayerState::Attacking && attack_timer.0.finished() {
        commands.spawn((
            Projectile,
            Sprite::from_color(Color::WHITE, Vec2::from((PROJECTILE_SIZE, PROJECTILE_SIZE))),
            Transform::from_translation(player_position),
            Velocity(
                PROJECTILE_SPEED
                    * (cursor_position.0.unwrap_or(SPRITE_ORIENTATION) - player_position.xy())
                        .normalize_or_zero(),
            ),
            StateScoped(MainState::Game),
            Lifetime::new(PROJECTILE_LIFETIME),
        ));
        attack_timer.0.reset();
    }

    Ok(())
}
