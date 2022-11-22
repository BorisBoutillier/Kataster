use bevy::app::AppExit;

use crate::prelude::*;

#[derive(Component)]
pub struct DrawBlinkTimer(pub Timer);

// List of user actions associated to menu/ui interaction
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum MenuAction {
    // Starts the game when in the start screen
    // Go to the start screen when in the game over screen
    Accept,
    // During gameplay, pause the game.
    // Also unpause the game when in the pause screen.
    PauseUnpause,
    // During gameplay, directly exit to the initial screen.
    ExitToMenu,
    // During non-gameplay screens, quit the game
    Quit,
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::StartMenu).with_system(start_menu))
            .add_system_set(SystemSet::on_enter(AppGameState::Pause).with_system(pause_menu))
            .add_system_set(SystemSet::on_enter(AppGameState::GameOver).with_system(gameover_menu))
            .add_system(menu_input_system)
            .add_system(menu_blink_system);
    }
}

pub fn start_menu(mut commands: Commands, assets: ResMut<GameAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
            ForState {
                states: vec![AppState::StartMenu],
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "Kataster",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 100.0,
                            color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                        },
                    ),
                    ..Default::default()
                },
                ForState {
                    states: vec![AppState::StartMenu],
                },
            ));
            parent.spawn((
                TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "enter",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 50.0,
                            color: Color::rgb_u8(0x00, 0x44, 0x44),
                        },
                    ),
                    ..Default::default()
                },
                DrawBlinkTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
                ForState {
                    states: vec![AppState::StartMenu],
                },
            ));
        });
}

pub fn gameover_menu(mut commands: Commands, assets: ResMut<GameAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
            ForState {
                states: vec![AppGameState::GameOver],
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "Game Over",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 100.0,
                            color: Color::rgb_u8(0xAA, 0x22, 0x22),
                        },
                    ),
                    ..Default::default()
                },
                ForState {
                    states: vec![AppGameState::GameOver],
                },
            ));
            parent.spawn((
                TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "enter",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 50.0,
                            color: Color::rgb_u8(0x88, 0x22, 0x22),
                        },
                    ),
                    ..Default::default()
                },
                DrawBlinkTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
                ForState {
                    states: vec![AppGameState::GameOver],
                },
            ));
        });
}

pub fn pause_menu(mut commands: Commands, assets: ResMut<GameAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            ForState {
                states: vec![AppGameState::Pause],
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "pause",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 100.0,
                            color: Color::rgb_u8(0xF8, 0xE4, 0x73),
                        },
                    ),
                    ..Default::default()
                },
                DrawBlinkTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
                ForState {
                    states: vec![AppGameState::Pause],
                },
            ));
        });
}

pub fn menu_blink_system(
    time: Res<Time>,
    mut query: Query<(&mut DrawBlinkTimer, &mut Visibility)>,
) {
    for (mut timer, mut visibility) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            visibility.is_visible = !visibility.is_visible;
        }
    }
}

pub fn menu_input_system(
    mut state: ResMut<State<AppState>>,
    mut gamestate: ResMut<State<AppGameState>>,
    menu_action_state: Res<ActionState<MenuAction>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if state.current() != &AppState::StartMenu
        && menu_action_state.just_pressed(MenuAction::ExitToMenu)
    {
        state.set(AppState::StartMenu).unwrap();
        gamestate.set(AppGameState::Invalid).unwrap();
        rapier_configuration.physics_pipeline_active = true;
    }
    if state.current() == &AppState::Game {
        if gamestate.current() == &AppGameState::Game {
            if menu_action_state.just_pressed(MenuAction::PauseUnpause) {
                gamestate.set(AppGameState::Pause).unwrap();
                rapier_configuration.physics_pipeline_active = false;
            }
        } else if gamestate.current() == &AppGameState::Pause {
            if menu_action_state.just_pressed(MenuAction::PauseUnpause) {
                gamestate.set(AppGameState::Game).unwrap();
                rapier_configuration.physics_pipeline_active = true;
            }
        } else if gamestate.current() == &AppGameState::GameOver {
            if menu_action_state.just_pressed(MenuAction::Accept) {
                state.set(AppState::StartMenu).unwrap();
                gamestate.set(AppGameState::Invalid).unwrap();
            }
            if menu_action_state.just_pressed(MenuAction::Quit) {
                app_exit_events.send(AppExit);
            }
        }
    } else if state.current() == &AppState::StartMenu {
        if menu_action_state.just_pressed(MenuAction::Accept) {
            state.set(AppState::Game).unwrap();
            gamestate.set(AppGameState::Game).unwrap();
        }
        if menu_action_state.just_pressed(MenuAction::Quit) {
            app_exit_events.send(AppExit);
        }
    }
}
