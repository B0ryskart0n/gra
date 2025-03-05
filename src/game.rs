use bevy::prelude::*;

use super::GameState;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), game_enter)
        // .add_systems(OnExit(GameState::Game), game_exit)
        .add_systems(Update, game_update.run_if(in_state(GameState::Game)));
}

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn game_enter(mut commands: Commands) {
    commands.spawn((Text2d::new("In game"), StateScoped(GameState::Game)));
    commands.insert_resource(GameTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn game_update(time: Res<Time>, mut next_state: ResMut<NextState<GameState>>, mut timer: ResMut<GameTimer>) {
    if timer.tick(time.delta()).finished() {
        next_state.set(GameState::Menu);
    }
}
