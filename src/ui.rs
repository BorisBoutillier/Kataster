use super::components::*;
use super::state::*;
use super::START_LIFE;
use bevy::prelude::*;

pub struct DrawBlinkTimer(pub Timer);

pub fn start_menu(
    commands: &mut Commands,
    runstate: ResMut<RunState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(NodeBundle {
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
        .with(ForState {
            states: vec![AppState::StartMenu],
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text {
                        value: "Kataster".to_string(),
                        font: runstate.font_handle.clone(),
                        style: TextStyle {
                            font_size: 100.0,
                            color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(ForState {
                    states: vec![AppState::StartMenu],
                })
                .spawn(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text {
                        value: "enter".to_string(),
                        font: runstate.font_handle.clone(),
                        style: TextStyle {
                            font_size: 50.0,
                            color: Color::rgb_u8(0x00, 0x44, 0x44),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(DrawBlinkTimer(Timer::from_seconds(0.5, true)))
                .with(ForState {
                    states: vec![AppState::StartMenu],
                });
        });
}

pub fn gameover_menu(
    commands: &mut Commands,
    runstate: ResMut<RunState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(NodeBundle {
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
        .with(ForState {
            states: vec![AppGameState::GameOver],
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text {
                        value: "Game Over".to_string(),
                        font: runstate.font_handle.clone(),
                        style: TextStyle {
                            font_size: 100.0,
                            color: Color::rgb_u8(0xAA, 0x22, 0x22),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(ForState {
                    states: vec![AppGameState::GameOver],
                })
                .spawn(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text {
                        value: "enter".to_string(),
                        font: runstate.font_handle.clone(),
                        style: TextStyle {
                            font_size: 50.0,
                            color: Color::rgb_u8(0x88, 0x22, 0x22),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(DrawBlinkTimer(Timer::from_seconds(0.5, true)))
                .with(ForState {
                    states: vec![AppGameState::GameOver],
                });
        });
}

pub fn pause_menu(
    commands: &mut Commands,
    runstate: ResMut<RunState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(NodeBundle {
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
        .with(ForState {
            states: vec![AppGameState::Pause],
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text {
                        value: "pause".to_string(),
                        font: runstate.font_handle.clone(),
                        style: TextStyle {
                            font_size: 100.0,
                            color: Color::rgb_u8(0xF8, 0xE4, 0x73),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(DrawBlinkTimer(Timer::from_seconds(0.5, true)))
                .with(ForState {
                    states: vec![AppGameState::Pause],
                });
        });
}

pub fn draw_blink_system(time: Res<Time>, mut query: Query<(Mut<DrawBlinkTimer>, Mut<Draw>)>) {
    for (mut timer, mut draw) in query.iter_mut() {
        timer.0.tick(time.delta_seconds());
        if timer.0.finished() {
            draw.is_visible = !draw.is_visible;
        }
    }
}

pub fn game_ui_spawn(
    commands: &mut Commands,
    runstate: ResMut<RunState>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(NodeBundle {
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
        .with(ForState {
            states: vec![AppState::Game],
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
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
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(ForState {
                    states: vec![AppState::Game],
                })
                .with(UiScore {});
        })
        // Life counters
        // Not kept in 'GameOver' state, simplifying last counter removal.
        .spawn(NodeBundle {
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
        .with(ForState {
            states: vec![AppState::Game],
        })
        .with_children(|parent| {
            for i in 1..(START_LIFE + 1) {
                parent
                    .spawn(ImageBundle {
                        style: Style {
                            margin: Rect {
                                left: Val::Px(10.0),
                                right: Val::Px(10.0),
                                top: Val::Px(10.0),
                                bottom: Val::Px(10.0),
                            },
                            ..Default::default()
                        },
                        material: materials.add(asset_server.load("playerLife1_red.png").into()),
                        draw: Draw {
                            is_transparent: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with(ForState {
                        states: vec![AppState::Game],
                    })
                    .with(UiLife { min: i });
            }
        });
}

pub fn score_ui_system(runstate: ChangedRes<RunState>, mut query: Query<Mut<Text>, With<UiScore>>) {
    for mut text in query.iter_mut() {
        text.value = format!("{}", runstate.score.unwrap());
    }
}
pub fn life_ui_system(
    runstate: Res<RunState>,
    ship_query: Query<&Ship>,
    mut uilife_query: Query<(Mut<Draw>, &UiLife)>,
) {
    let mut life = 0;
    if let Some(player) = runstate.player {
        if let Ok(ship) = ship_query.get_component::<Ship>(player) {
            life = ship.life;
        }
    }
    for (mut draw, uilife) in uilife_query.iter_mut() {
        draw.is_visible = life >= uilife.min;
    }
}
