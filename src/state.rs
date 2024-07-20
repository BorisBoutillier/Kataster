use crate::prelude::*;
use enum_iterator::{all, Sequence};

// Main state enum, differianting, Menu from Game 'scenes'
#[derive(States, Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Sequence)]
pub enum AppState {
    #[default]
    Setup,
    Menu,
    Game,
}
#[derive(SubStates, Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Sequence)]
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
            transition_setup_to_menu.run_if(in_state(AppState::Setup)),
        );
    }
}

fn transition_setup_to_menu(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::Menu);
}
