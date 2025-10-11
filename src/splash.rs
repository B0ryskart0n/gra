use super::MainState;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MainState::Splash), setup)
        .add_systems(Update, splash_update.run_if(in_state(MainState::Splash)));
}

#[derive(Component, Deref, DerefMut)]
struct SplashTimer(Timer);

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn((
        Text2d::new("Splash screen"),
        TextLayout::new_with_justify(Justify::Center),
        DespawnOnExit(MainState::Splash),
    ));
    commands.spawn((
        // In the future splash screen should show something for more than 0 seconds.
        SplashTimer(Timer::from_seconds(0.0, TimerMode::Once)),
        DespawnOnExit(MainState::Splash),
    ));
}

fn splash_update(
    time: Res<Time>,
    mut next_state: ResMut<NextState<MainState>>,
    mut q_timer: Query<&mut SplashTimer>,
) -> Result {
    if q_timer.single_mut()?.0.tick(time.delta()).is_finished() {
        next_state.set(MainState::Menu);
    }
    Ok(())
}
