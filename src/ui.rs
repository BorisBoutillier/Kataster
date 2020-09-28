use super::state::*;
use bevy::prelude::*;

pub struct DrawBlinkTimer(pub Timer);

pub fn start_menu(
    mut commands: Commands,
    runstate: ResMut<RunState>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if runstate.gamestate.entering(GameState::StartMenu) {
        let font_handle = asset_server.load("assets/kenvector_future.ttf").unwrap();
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
                            font: font_handle,
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
                            font: font_handle,
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
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if runstate.gamestate.entering(GameState::GameOver) {
        let font_handle = asset_server.load("assets/kenvector_future.ttf").unwrap();
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
                            font: font_handle,
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
                            font: font_handle,
                            style: TextStyle {
                                font_size: 50.0,
                                color: Color::rgb_u8(0x44, 0x11, 0x11),
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
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if runstate.gamestate.entering(GameState::Pause) {
        let font_handle = asset_server.load("assets/kenvector_future.ttf").unwrap();
        commands
            .spawn(NodeComponents {
                style: Style {
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
                            font: font_handle,
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
