use super::arena::*;
use bevy::prelude::*;
/// Component to tag an entity as only needed in one state
pub struct ForState<T> {
    pub states: Vec<T>,
}

pub const APPSTATE_STAGE: &str = "appstate_stage";
pub const APPGAMESTATE_STAGE: &str = "appgamestate_stage";

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
    pub laser_texture_handle: Handle<ColorMaterial>,
    pub laser_audio_handle: Handle<AudioSource>,
    pub meteor_big_handle: Handle<ColorMaterial>,
    pub meteor_med_handle: Handle<ColorMaterial>,
    pub meteor_small_handle: Handle<ColorMaterial>,
}

impl RunState {
    pub fn new(
        asset_server: &AssetServer,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> RunState {
        RunState {
            player: None,
            arena: None,
            score: None,
            font_handle: asset_server.load("kenvector_future.ttf"),
            laser_texture_handle: materials.add(asset_server.load("laserRed07.png").into()),
            laser_audio_handle: asset_server.load("sfx_laser1.mp3"),
            meteor_big_handle: materials.add(asset_server.load("meteorBrown_big1.png").into()),
            meteor_med_handle: materials.add(asset_server.load("meteorBrown_med1.png").into()),
            meteor_small_handle: materials.add(asset_server.load("meteorBrown_small1.png").into()),
        }
    }
}

pub fn appstate_exit_despawn(
    commands: &mut Commands,
    state: Res<State<AppState>>,
    query: Query<(Entity, &ForState<AppState>)>,
) {
    for (entity, for_state) in &mut query.iter() {
        if !for_state.states.contains(&state.current()) {
            commands.despawn(entity);
        }
    }
}

pub fn appgamestate_exit_despawn(
    commands: &mut Commands,
    state: ResMut<State<AppGameState>>,
    query: Query<(Entity, &ForState<AppGameState>)>,
) {
    for (entity, for_state) in &mut query.iter() {
        if !for_state.states.contains(&state.current()) {
            commands.despawn(entity);
        }
    }
}
