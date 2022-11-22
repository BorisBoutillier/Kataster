use bevy::ecs::schedule::StateData;

use crate::prelude::*;

/// Component to tag an entity as only needed in some of the states
#[derive(Component)]
pub struct ForState<T> {
    pub states: Vec<T>,
}

// Main state enum, differianting, Menu from Game 'scenes'
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AppState {
    StartMenu,
    Game,
}

// Game state enum, differianting several phase of the game
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AppGameState {
    /// Invalid used when AppState is NOT Game
    Invalid,
    Game,
    Pause,
    GameOver,
}

#[derive(Debug, Resource)]
pub struct RunState {
    pub arena: Option<Arena>,
    pub score: Option<u32>,
    // Store the most used asset handles
    pub font_handle: Handle<Font>,
    pub laser_texture_handle: Handle<Image>,
    pub laser_audio_handle: Handle<AudioSource>,
    pub meteor_big_handle: Handle<Image>,
    pub meteor_med_handle: Handle<Image>,
    pub meteor_small_handle: Handle<Image>,
}

impl RunState {
    pub fn new(asset_server: &AssetServer) -> RunState {
        RunState {
            arena: None,
            score: None,
            font_handle: asset_server.load("kenvector_future.ttf"),
            laser_texture_handle: asset_server.load("laserRed07.png"),
            laser_audio_handle: asset_server.load("sfx_laser1.ogg"),
            meteor_big_handle: asset_server.load("meteorBrown_big1.png"),
            meteor_med_handle: asset_server.load("meteorBrown_med1.png"),
            meteor_small_handle: asset_server.load("meteorBrown_small1.png"),
        }
    }
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        for state in [AppState::StartMenu, AppState::Game].into_iter() {
            app.add_system_set(
                SystemSet::on_enter(state).with_system(state_enter_despawn::<AppState>),
            );
        }
        for state in [
            AppGameState::Invalid,
            AppGameState::Game,
            AppGameState::Pause,
            AppGameState::GameOver,
        ]
        .into_iter()
        {
            app.add_system_set(
                SystemSet::on_enter(state).with_system(state_enter_despawn::<AppGameState>),
            );
        }
    }
}

pub fn state_enter_despawn<T: StateData>(
    mut commands: Commands,
    state: ResMut<State<T>>,
    query: Query<(Entity, &ForState<T>)>,
) {
    for (entity, for_state) in &mut query.iter() {
        if !for_state.states.contains(state.current()) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
