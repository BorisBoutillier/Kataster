use core::fmt;

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
    pub gamestate: GameStateFsm<GameState>,
    pub player: Option<Entity>,
    pub arena: Option<Arena>,
    pub score: Option<u32>,
}

impl RunState {
    pub fn new(start: GameState) -> RunState {
        RunState {
            gamestate: GameStateFsm::new(start),
            player: None,
            arena: None,
            score: None,
        }
    }
}

pub fn runstate_fsm(mut runstate: ResMut<RunState>) {
    runstate.gamestate.update();
}

pub fn state_exit_despawn(
    mut commands: Commands,
    runstate: ResMut<RunState>,
    mut query: Query<(Entity, &ForStates)>,
) {
    for (entity, for_states) in &mut query.iter() {
        if runstate.gamestate.exiting_one_of(&for_states.states)
            && !runstate.gamestate.transiting_to_one_of(&for_states.states)
        {
            commands.despawn(entity);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum FsmTransition {
    Exit,
    Enter,
    None,
}
#[derive(Debug)]
pub struct GameStateFsm<T: PartialEq + Eq + Copy + fmt::Debug> {
    current: Option<T>,
    transition: FsmTransition,
    next: Option<T>,
    prev: Option<T>,
}

impl<T: PartialEq + Eq + Copy + fmt::Debug> GameStateFsm<T> {
    pub fn new(start: T) -> GameStateFsm<T> {
        GameStateFsm {
            current: None,
            transition: FsmTransition::Enter,
            next: Some(start),
            prev: None,
        }
    }
    pub fn is(&self, state: T) -> bool {
        self.current == Some(state)
    }
    pub fn exiting_one_of(&self, states: &[T]) -> bool {
        self.transition == FsmTransition::Exit && states.contains(&self.current.unwrap())
    }
    pub fn transiting_to_one_of(&self, states: &[T]) -> bool {
        self.next
            .map(|next| states.contains(&next))
            .unwrap_or(false)
    }
    pub fn entering(&self, state: T) -> bool {
        self.transition == FsmTransition::Enter && self.next == Some(state)
    }
    pub fn entering_not_from(&self, state: T, from: T) -> bool {
        self.transition == FsmTransition::Enter
            && self.next == Some(state)
            && self.prev != Some(from)
    }
    pub fn transit_to(&mut self, state: T) {
        self.next = Some(state);
    }
    /// Called every frame to update the phases of transitions.
    /// A transition requires 3 frames: Exit current, enter next, current=next
    pub fn update(&mut self) {
        if self.next.is_some() {
            match self.transition {
                FsmTransition::Exit => {
                    // We have exited current state, we can enter the new one
                    self.prev = self.current;
                    self.current = None;
                    self.transition = FsmTransition::Enter;
                }
                FsmTransition::Enter => {
                    // We have entered the new one it is now current
                    self.current = self.next;
                    self.transition = FsmTransition::None;
                    self.next = None;
                }
                FsmTransition::None => {
                    // This is new request to go to the next state, exit the current one first
                    self.transition = FsmTransition::Exit;
                }
            }
            //println!("After Update {:?}", self);
        }
    }
}
