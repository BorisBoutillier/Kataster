use crate::prelude::*;

pub fn main_menu_input_system(
    app_state: ResMut<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    menu_action_state: Res<ActionState<MenuAction>>,
    mut app_exit_events: EventWriter<AppExit>,
    menu: Query<&MenuHandler>,
) {
    if let Ok(menu) = menu.single() {
        if menu_action_state.just_pressed(&MenuAction::Accept) {
            if app_state.get() == &AppState::Menu {
                match menu.selected_id {
                    0 => {
                        next_app_state.set(AppState::Game);
                    }
                    1 => {
                        next_app_state.set(AppState::Credits);
                    }
                    _ => {
                        app_exit_events.write(AppExit::Success);
                    }
                }
            }
            if app_state.get() == &AppState::Credits {
                match menu.selected_id {
                    0 => {
                        next_app_state.set(AppState::Menu);
                    }
                    _ => {
                        app_exit_events.write(AppExit::Success);
                    }
                }
            }
        }
    }
}

pub fn game_menu_input_system(
    game_state: ResMut<State<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    menu_action_state: Res<ActionState<MenuAction>>,
    mut app_exit_events: EventWriter<AppExit>,
    menu: Query<&MenuHandler>,
) {
    if menu_action_state.just_pressed(&MenuAction::PauseUnpause) {
        if game_state.get() == &GameState::Running {
            next_game_state.set(GameState::Paused);
        }
        if game_state.get() == &GameState::Paused {
            next_game_state.set(GameState::Running);
        }
    }
    if let Ok(menu) = menu.single() {
        if menu_action_state.just_pressed(&MenuAction::Accept) {
            if game_state.get() == &GameState::Paused {
                match menu.selected_id {
                    0 => {
                        next_game_state.set(GameState::Running);
                    }
                    1 => {
                        next_app_state.set(AppState::Menu);
                    }
                    _ => {
                        app_exit_events.write(AppExit::Success);
                    }
                }
            }
            if game_state.get() == &GameState::Over {
                match menu.selected_id {
                    0 => {
                        next_app_state.set(AppState::Menu);
                    }
                    _ => {
                        app_exit_events.write(AppExit::Success);
                    }
                }
            }
        }
    }
}
