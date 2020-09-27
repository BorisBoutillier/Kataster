use super::arena::*;
use bevy::prelude::*;
/// Component to tag an entity as only needed in one game state
pub struct ForStates {
    pub states: Vec<GameState>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameState {
    StartMenu,
    Game,
    GameOver,
    Pause,
}
#[derive(Debug)]
pub struct RunState {
    pub current: Option<GameState>,
    pub enter: bool,
    pub exit: bool,
    pub next: Option<GameState>,
    pub prev: Option<GameState>,
    pub player: Option<Entity>,
    pub arena: Option<Arena>,
}

impl RunState {
    pub fn new(next: GameState) -> RunState {
        RunState {
            current: None,
            enter: true,
            exit: false,
            next: Some(next),
            prev: None,
            player: None,
            arena: None,
        }
    }
}

pub fn runstate_fsm(mut runstate: ResMut<RunState>) {
    if runstate.next.is_some() {
        if runstate.exit {
            // We have exited current state, we can enter the new one
            runstate.prev = runstate.current;
            runstate.current = None;
            runstate.enter = true;
            runstate.exit = false;
        } else if runstate.enter {
            // We have enter the new one it is now current
            runstate.current = runstate.next;
            runstate.enter = false;
            runstate.next = None;
        } else {
            // This is new request to go to the next state, exit the current one first
            runstate.exit = true;
        }
        //println!("Runstate: {:?}", *runstate);
    }
}

pub fn state_exit_despawn(
    mut commands: Commands,
    runstate: ResMut<RunState>,
    mut query: Query<(Entity, &ForStates)>,
) {
    if runstate.exit {
        let current = runstate.current.unwrap();
        let next = runstate.next.unwrap();
        for (entity, for_states) in &mut query.iter() {
            if for_states.states.contains(&current) && !for_states.states.contains(&next) {
                commands.despawn(entity);
            }
        }
    }
}
