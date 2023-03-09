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
        app.add_system(start_menu.in_schedule(OnEnter(AppState::StartMenu)))
            .add_system(pause_menu.in_schedule(OnEnter(AppGameState::Pause)))
            .add_system(gameover_menu.in_schedule(OnEnter(AppGameState::GameOver)))
            .add_system(menu_input_system)
            .add_system(menu_blink_system)
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    let mut input_map = InputMap::<MenuAction>::new([
        (KeyCode::Return, MenuAction::Accept),
        (KeyCode::Escape, MenuAction::PauseUnpause),
        (KeyCode::Back, MenuAction::ExitToMenu),
        (KeyCode::Escape, MenuAction::Quit),
    ]);
    input_map.insert(GamepadButtonType::Select, MenuAction::ExitToMenu);
    input_map.insert(GamepadButtonType::Start, MenuAction::PauseUnpause);
    input_map.insert(GamepadButtonType::South, MenuAction::Accept);
    input_map.insert(GamepadButtonType::East, MenuAction::Quit);
    // Insert MenuAction resources
    commands.insert_resource(input_map);
    commands.insert_resource(ActionState::<MenuAction>::default());
}

fn start_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
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
            parent.spawn((TextBundle {
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
            },));
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
            ));
        });
}

fn gameover_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
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
            parent.spawn((TextBundle {
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
            },));
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
            ));
        });
}

fn pause_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
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
            ));
        });
}

fn menu_blink_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DrawBlinkTimer, &ComputedVisibility)>,
) {
    for (entity, mut timer, visibility) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            let new_visibility = if visibility.is_visible() {
                Visibility::Hidden
            } else {
                Visibility::Visible
            };
            commands.entity(entity).insert(new_visibility);
        }
    }
}

fn menu_input_system(
    state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    gamestate: Res<State<AppGameState>>,
    mut next_gamestate: ResMut<NextState<AppGameState>>,
    menu_action_state: Res<ActionState<MenuAction>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if state.0 != AppState::StartMenu && menu_action_state.just_pressed(MenuAction::ExitToMenu) {
        next_state.set(AppState::StartMenu);
        next_gamestate.set(AppGameState::Invalid);
        rapier_configuration.physics_pipeline_active = true;
    }
    if state.0 == AppState::Game {
        if gamestate.0 == AppGameState::Game {
            if menu_action_state.just_pressed(MenuAction::PauseUnpause) {
                next_gamestate.set(AppGameState::Pause);
                rapier_configuration.physics_pipeline_active = false;
            }
        } else if gamestate.0 == AppGameState::Pause {
            if menu_action_state.just_pressed(MenuAction::PauseUnpause) {
                next_gamestate.set(AppGameState::Game);
                rapier_configuration.physics_pipeline_active = true;
            }
        } else if gamestate.0 == AppGameState::GameOver {
            if menu_action_state.just_pressed(MenuAction::Accept) {
                next_state.set(AppState::StartMenu);
                next_gamestate.set(AppGameState::Invalid);
            }
            if menu_action_state.just_pressed(MenuAction::Quit) {
                app_exit_events.send(AppExit);
            }
        }
    } else if state.0 == AppState::StartMenu {
        if menu_action_state.just_pressed(MenuAction::Accept) {
            next_state.set(AppState::Game);
            next_gamestate.set(AppGameState::Game);
        }
        if menu_action_state.just_pressed(MenuAction::Quit) {
            app_exit_events.send(AppExit);
        }
    }
}
