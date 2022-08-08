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

pub fn start_menu(mut commands: Commands, runstate: ResMut<RunState>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(ForState {
            states: vec![AppState::StartMenu],
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "Kataster",
                        TextStyle {
                            font: runstate.font_handle.clone(),
                            font_size: 100.0,
                            color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                        },
                    ),
                    ..Default::default()
                })
                .insert(ForState {
                    states: vec![AppState::StartMenu],
                });
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "enter",
                        TextStyle {
                            font: runstate.font_handle.clone(),
                            font_size: 50.0,
                            color: Color::rgb_u8(0x00, 0x44, 0x44),
                        },
                    ),
                    ..Default::default()
                })
                .insert(DrawBlinkTimer(Timer::from_seconds(0.5, true)))
                .insert(ForState {
                    states: vec![AppState::StartMenu],
                });
        });
}

pub fn gameover_menu(mut commands: Commands, runstate: ResMut<RunState>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(ForState {
            states: vec![AppGameState::GameOver],
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "Game Over",
                        TextStyle {
                            font: runstate.font_handle.clone(),
                            font_size: 100.0,
                            color: Color::rgb_u8(0xAA, 0x22, 0x22),
                        },
                    ),
                    ..Default::default()
                })
                .insert(ForState {
                    states: vec![AppGameState::GameOver],
                });
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "enter",
                        TextStyle {
                            font: runstate.font_handle.clone(),
                            font_size: 50.0,
                            color: Color::rgb_u8(0x88, 0x22, 0x22),
                        },
                    ),
                    ..Default::default()
                })
                .insert(DrawBlinkTimer(Timer::from_seconds(0.5, true)))
                .insert(ForState {
                    states: vec![AppGameState::GameOver],
                });
        });
}

pub fn pause_menu(mut commands: Commands, runstate: ResMut<RunState>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(ForState {
            states: vec![AppGameState::Pause],
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "pause",
                        TextStyle {
                            font: runstate.font_handle.clone(),
                            font_size: 100.0,
                            color: Color::rgb_u8(0xF8, 0xE4, 0x73),
                        },
                    ),
                    ..Default::default()
                })
                .insert(DrawBlinkTimer(Timer::from_seconds(0.5, true)))
                .insert(ForState {
                    states: vec![AppGameState::Pause],
                });
        });
}

pub fn draw_blink_system(
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

pub fn game_ui_spawn(
    mut commands: Commands,
    runstate: ResMut<RunState>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexEnd,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(ForState {
            states: vec![AppState::Game],
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        justify_content: JustifyContent::FlexEnd,
                        margin: UiRect {
                            left: Val::Px(10.0),
                            right: Val::Px(10.0),
                            top: Val::Px(10.0),
                            bottom: Val::Px(10.0),
                        },
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "0",
                        TextStyle {
                            font: runstate.font_handle.clone(),
                            font_size: 50.0,
                            color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                        },
                    ),
                    ..Default::default()
                })
                .insert(ForState {
                    states: vec![AppState::Game],
                })
                .insert(UiScore {});
        });
    // Life counters
    // Not kept in 'GameOver' state, simplifying last counter removal.
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(ForState {
            states: vec![AppState::Game],
        })
        .with_children(|parent| {
            for i in 1..(START_LIFE + 1) {
                parent
                    .spawn_bundle(ImageBundle {
                        style: Style {
                            margin: UiRect {
                                left: Val::Px(10.0),
                                right: Val::Px(10.0),
                                top: Val::Px(10.0),
                                bottom: Val::Px(10.0),
                            },
                            ..Default::default()
                        },
                        image: asset_server.load("playerLife1_red.png").into(),
                        ..Default::default()
                    })
                    .insert(ForState {
                        states: vec![AppState::Game],
                    })
                    .insert(UiLife { min: i });
            }
        });
}

pub fn score_ui_system(runstate: Res<RunState>, mut query: Query<&mut Text, With<UiScore>>) {
    if runstate.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{}", runstate.score.unwrap());
        }
    }
}
pub fn life_ui_system(
    runstate: Res<RunState>,
    ship_query: Query<&Ship>,
    mut uilife_query: Query<(&mut Visibility, &UiLife)>,
) {
    let mut life = 0;
    if let Some(player) = runstate.player {
        if let Ok(ship) = ship_query.get_component::<Ship>(player) {
            life = ship.life;
        }
    }
    for (mut visibility, uilife) in uilife_query.iter_mut() {
        visibility.is_visible = life >= uilife.min;
    }
}

pub fn ui_input_system(
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
