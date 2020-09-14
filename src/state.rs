use super::arena::*;
use bevy::prelude::*;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameState {
    MainMenu,
    Game,
}
#[derive(Debug)]
pub struct RunState {
    pub current: Option<GameState>,
    pub enter: bool,
    pub exit: bool,
    pub next: Option<GameState>,
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
            player: None,
            arena: None,
        }
    }
}

pub fn runstate_fsm(mut runstate: ResMut<RunState>) {
    if runstate.next.is_some() {
        if runstate.exit {
            // We have exited current state, we can enter the new one
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
        println!("Runstate: {:?}", *runstate);
    }
}
