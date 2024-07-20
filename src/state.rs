use crate::prelude::*;
use enum_iterator::{all, Sequence};

/// Component to tag an entity as only needed in some of the states
#[derive(Component, Debug)]
pub struct ForState<T> {
    pub states: Vec<T>,
}

// Main state enum, differianting, Menu from Game 'scenes'
#[derive(States, Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Sequence)]
pub enum AppState {
    #[default]
    Setup,
    StartMenu,
    GameCreate,
    GameRunning,
    GamePaused,
    GameOver,
}
impl AppState {
    pub const ANY_GAME_STATE: [AppState; 4] = [
        AppState::GameCreate,
        AppState::GameRunning,
        AppState::GamePaused,
        AppState::GameOver,
    ];
    pub fn is_any_game_state(&self) -> bool {
        AppState::ANY_GAME_STATE.contains(self)
    }
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        for state in all::<AppState>() {
            app.add_systems(OnEnter(state), state_enter_despawn::<AppState>);
        }
        app.add_systems(
            Update,
            transition_setup_to_menu.run_if(in_state(AppState::Setup)),
        );
    }
}

fn state_enter_despawn<T: States>(
    mut commands: Commands,
    state: ResMut<State<T>>,
    query: Query<(Entity, &ForState<T>)>,
) {
    for (entity, for_state) in &mut query.iter() {
        if !for_state.states.contains(state.get()) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
fn transition_setup_to_menu(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::StartMenu);
}
