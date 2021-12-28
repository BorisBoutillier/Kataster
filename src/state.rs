use crate::prelude::*;

/// Component to tag an entity as only needed in one state
#[derive(Component)]
pub struct ForState<T> {
    pub states: Vec<T>,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AppState {
    StartMenu,
    Game,
}
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AppGameState {
    /// Invalid used when AppState is NOT Game
    Invalid,
    Game,
    Pause,
    GameOver,
}

#[derive(Debug)]
pub struct RunState {
    pub player: Option<Entity>,
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
            player: None,
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

pub fn appstate_enter_despawn(
    mut commands: Commands,
    state: Res<State<AppState>>,
    query: Query<(Entity, &ForState<AppState>)>,
) {
    for (entity, for_state) in &mut query.iter() {
        if !for_state.states.contains(state.current()) {
            commands.entity(entity).despawn();
        }
    }
}

pub fn appgamestate_enter_despawn(
    mut commands: Commands,
    state: ResMut<State<AppGameState>>,
    query: Query<(Entity, &ForState<AppGameState>)>,
) {
    for (entity, for_state) in &mut query.iter() {
        if !for_state.states.contains(state.current()) {
            commands.entity(entity).despawn();
        }
    }
}
