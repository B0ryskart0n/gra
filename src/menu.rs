use super::GameState;
use super::utils::*;
use bevy::prelude::*;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), menu_enter)
        .add_systems(Update, menu_update.run_if(in_state(GameState::Menu)));
}

#[derive(Resource, Deref, DerefMut)]
struct MenuTimer(Timer);

fn menu_enter(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut camera = camera_query.single_mut();
    camera.translation = ZERO3;
    commands.spawn((Text2d::new("Main menu"), StateScoped(GameState::Menu)));
    commands.insert_resource(MenuTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

fn menu_update(time: Res<Time>, mut next_state: ResMut<NextState<GameState>>, mut timer: ResMut<MenuTimer>) {
    if timer.tick(time.delta()).finished() {
        next_state.set(GameState::Game);
    }
}
