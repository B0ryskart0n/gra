use super::GameState;
use bevy::prelude::*;

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), splash_enter)
        // .add_systems(OnExit(GameState::Splash), splash_exit)
        .add_systems(Update, splash_update.run_if(in_state(GameState::Splash)));
}

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_enter(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn((
        Text2d::new("Splash screen"),
        TextLayout::new_with_justify(JustifyText::Center),
        StateScoped(GameState::Splash),
    ));
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

fn splash_update(time: Res<Time>, mut next_state: ResMut<NextState<GameState>>, mut timer: ResMut<SplashTimer>) {
    if timer.tick(time.delta()).finished() {
        next_state.set(GameState::Menu);
    }
}
