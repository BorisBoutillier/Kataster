use super::components::*;
use super::state::*;
use super::START_LIFE;
use bevy::prelude::*;

pub struct DrawBlinkTimer(pub Timer);

pub fn start_menu(
    mut commands: Commands,
    runstate: ResMut<RunState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if runstate.gamestate.entering(GameState::StartMenu) {
        commands
            .spawn(NodeComponents {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                draw: Draw {
                    is_transparent: true,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(ForStates {
                states: vec![GameState::StartMenu],
            })
            .with_children(|parent| {
                parent
                    .spawn(TextComponents {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text {
                            value: "Kataster".to_string(),
                            font: runstate.font_handle.clone(),
                            style: TextStyle {
                                font_size: 100.0,
                                color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                            },
                        },
                        ..Default::default()
                    })
                    .with(ForStates {
                        states: vec![GameState::StartMenu],
                    })
                    .spawn(TextComponents {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text {
                            value: "enter".to_string(),
                            font: runstate.font_handle.clone(),
                            style: TextStyle {
                                font_size: 50.0,
                                color: Color::rgb_u8(0x00, 0x44, 0x44),
                            },
                        },
                        ..Default::default()
                    })
                    .with(DrawBlinkTimer(Timer::from_seconds(0.5, true)))
                    .with(ForStates {
                        states: vec![GameState::StartMenu],
                    });
            });
    }
}

pub fn gameover_menu(
    mut commands: Commands,
    runstate: ResMut<RunState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if runstate.gamestate.entering(GameState::GameOver) {
        commands
            .spawn(NodeComponents {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                draw: Draw {
                    is_transparent: true,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(ForStates {
                states: vec![GameState::GameOver],
            })
            .with_children(|parent| {
                parent
                    .spawn(TextComponents {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text {
                            value: "Game Over".to_string(),
                            font: runstate.font_handle.clone(),
                            style: TextStyle {
                                font_size: 100.0,
                                color: Color::rgb_u8(0xAA, 0x22, 0x22),
                            },
                        },
                        ..Default::default()
                    })
                    .with(ForStates {
                        states: vec![GameState::GameOver],
                    })
                    .spawn(TextComponents {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text {
                            value: "enter".to_string(),
                            font: runstate.font_handle.clone(),
                            style: TextStyle {
                                font_size: 50.0,
                                color: Color::rgb_u8(0x88, 0x22, 0x22),
                            },
                        },
                        ..Default::default()
                    })
                    .with(DrawBlinkTimer(Timer::from_seconds(0.5, true)))
                    .with(ForStates {
                        states: vec![GameState::GameOver],
                    });
            });
    }
}

pub fn pause_menu(
    mut commands: Commands,
    runstate: ResMut<RunState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if runstate.gamestate.entering(GameState::Pause) {
        commands
            .spawn(NodeComponents {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                draw: Draw {
                    is_transparent: true,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(ForStates {
                states: vec![GameState::Pause],
            })
            .with_children(|parent| {
                parent
                    .spawn(TextComponents {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text {
                            value: "pause".to_string(),
                            font: runstate.font_handle.clone(),
                            style: TextStyle {
                                font_size: 100.0,
                                color: Color::rgb_u8(0xF8, 0xE4, 0x73),
                            },
                        },
                        ..Default::default()
                    })
                    .with(DrawBlinkTimer(Timer::from_seconds(0.5, true)))
                    .with(ForStates {
                        states: vec![GameState::Pause],
                    });
            });
    }
}

pub fn draw_blink_system(time: Res<Time>, mut timer: Mut<DrawBlinkTimer>, mut draw: Mut<Draw>) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        draw.is_visible = !draw.is_visible;
    }
}

pub fn game_ui_spawn(
    mut commands: Commands,
    runstate: ResMut<RunState>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if runstate
        .gamestate
        .entering_not_from(GameState::Game, GameState::Pause)
    {
        commands
            .spawn(NodeComponents {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::FlexEnd,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                draw: Draw {
                    is_transparent: true,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(ForStates {
                states: vec![GameState::Game, GameState::Pause, GameState::GameOver],
            })
            .with_children(|parent| {
                parent
                    .spawn(TextComponents {
                        style: Style {
                            justify_content: JustifyContent::FlexEnd,
                            margin: Rect {
                                left: Val::Px(10.0),
                                right: Val::Px(10.0),
                                top: Val::Px(10.0),
                                bottom: Val::Px(10.0),
                            },
                            ..Default::default()
                        },
                        text: Text {
                            value: "0".to_string(),
                            font: runstate.font_handle.clone(),
                            style: TextStyle {
                                font_size: 50.0,
                                color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                            },
                        },
                        ..Default::default()
                    })
                    .with(ForStates {
                        states: vec![GameState::Game, GameState::Pause, GameState::GameOver],
                    })
                    .with(UiScore {});
            })
            // Life counters
            // Not kept in 'GameOver' state, simplifying last counter removal.
            .spawn(NodeComponents {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::FlexStart,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                draw: Draw {
                    is_transparent: true,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(ForStates {
                states: vec![GameState::Game, GameState::Pause],
            })
            .with_children(|parent| {
                for i in 1..(START_LIFE + 1) {
                    parent
                        .spawn(ImageComponents {
                            style: Style {
                                margin: Rect {
                                    left: Val::Px(10.0),
                                    right: Val::Px(10.0),
                                    top: Val::Px(10.0),
                                    bottom: Val::Px(10.0),
                                },
                                ..Default::default()
                            },
                            material: materials
                                .add(asset_server.load("playerLife1_red.png").into()),
                            draw: Draw {
                                is_transparent: true,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with(ForStates {
                            states: vec![GameState::Game, GameState::Pause],
                        })
                        .with(UiLife { min: i });
                }
            });
    }
}

pub fn score_ui_system(runstate: ChangedRes<RunState>, mut text: Mut<Text>, _uiscore: &UiScore) {
    if runstate.gamestate.is(GameState::Game) {
        text.value = format!("{}", runstate.score.unwrap());
    }
}
pub fn life_ui_system(
    runstate: Res<RunState>,
    ship_query: Query<&Ship>,
    mut uilife_query: Query<(Mut<Draw>, &UiLife)>,
) {
    if runstate.gamestate.is(GameState::Game) {
        if let Some(player) = runstate.player {
            if let Ok(ship) = ship_query.get_component::<Ship>(player) {
                for (mut draw, uilife) in uilife_query.iter_mut() {
                    draw.is_visible = ship.life >= uilife.min;
                }
            }
        }
    }
}
