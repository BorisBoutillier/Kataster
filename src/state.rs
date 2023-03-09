use crate::prelude::*;

/// Component to tag an entity as only needed in some of the states
#[derive(Component, Debug)]
pub struct ForState<T> {
    pub states: Vec<T>,
}

// Main state enum, differianting, Menu from Game 'scenes'
#[derive(States, Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub enum AppState {
    #[default]
    StartMenu,
    Game,
}

// Game state enum, differianting several phase of the game
#[derive(States, Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub enum AppGameState {
    /// Invalid used when AppState is NOT Game
    #[default]
    Invalid,
    Game,
    Pause,
    GameOver,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        for state in [AppState::StartMenu, AppState::Game].into_iter() {
            app.add_system(state_enter_despawn::<AppState>.in_schedule(OnEnter(state)));
        }
        for state in [
            AppGameState::Invalid,
            AppGameState::Game,
            AppGameState::Pause,
            AppGameState::GameOver,
        ]
        .into_iter()
        {
            app.add_system(state_enter_despawn::<AppGameState>.in_schedule(OnEnter(state)));
        }
    }
}

fn state_enter_despawn<T: States>(
    mut commands: Commands,
    state: ResMut<State<T>>,
    query: Query<(Entity, &ForState<T>)>,
) {
    for (entity, for_state) in &mut query.iter() {
        if !for_state.states.contains(&state.0) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
