use crate::prelude::*;

// Main state enum, differianting, Menu from Game 'scenes'
#[derive(States, Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub enum AppState {
    #[default]
    Setup,
    Menu,
    Game,
}
#[derive(SubStates, Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
#[source(AppState=AppState::Game)]
pub enum GameState {
    #[default]
    Setup,
    Running,
    Paused,
    Over,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
        app.enable_state_scoped_entities::<AppState>();
        app.add_sub_state::<GameState>();
        app.enable_state_scoped_entities::<GameState>();
        app.add_systems(
            Update,
            (
                transition_app_setup_to_menu.run_if(in_state(AppState::Setup)),
                transition_game_setup_to_running.run_if(in_state(GameState::Setup)),
            ),
        );
    }
}

fn transition_app_setup_to_menu(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::Menu);
}
fn transition_game_setup_to_running(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Running);
}
