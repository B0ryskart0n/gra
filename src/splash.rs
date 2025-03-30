use super::GameState;
use bevy::prelude::*;

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), enter_splash)
        .add_systems(Update, splash_update.run_if(in_state(GameState::Splash)))
        .add_systems(OnExit(GameState::Splash), exit_splash);
}

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn enter_splash(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn((
        Text2d::new("Splash screen"),
        TextLayout::new_with_justify(JustifyText::Center),
        StateScoped(GameState::Splash),
    ));
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}
fn exit_splash(mut commands: Commands) {
    commands.remove_resource::<SplashTimer>();
}

fn splash_update(time: Res<Time>, mut next_state: ResMut<NextState<GameState>>, mut timer: ResMut<SplashTimer>) {
    if timer.tick(time.delta()).finished() {
        next_state.set(GameState::Menu);
    }
}
