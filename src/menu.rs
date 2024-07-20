use bevy::app::AppExit;

use crate::prelude::*;

#[derive(Component)]
pub struct DrawBlinkTimer(pub Timer);

// List of user actions associated to menu/ui interaction
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
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
        app.add_systems(OnEnter(AppState::Setup), setup)
            .add_systems(OnEnter(AppState::Menu), start_menu)
            .add_systems(OnEnter(GameState::Paused), pause_menu)
            .add_systems(OnEnter(GameState::Over), gameover_menu)
            .add_systems(Update, (menu_input_system, menu_blink_system))
            .add_systems(
                Update,
                in_game_menu_input_system.run_if(in_state(AppState::Game)),
            );
    }
}

fn setup(mut commands: Commands) {
    let mut input_map = InputMap::<MenuAction>::new([
        (MenuAction::Accept, KeyCode::Enter),
        (MenuAction::PauseUnpause, KeyCode::Escape),
        (MenuAction::ExitToMenu, KeyCode::Backspace),
        (MenuAction::Quit, KeyCode::Escape),
    ]);
    input_map.insert(MenuAction::ExitToMenu, GamepadButtonType::Select);
    input_map.insert(MenuAction::PauseUnpause, GamepadButtonType::Start);
    input_map.insert(MenuAction::Accept, GamepadButtonType::South);
    input_map.insert(MenuAction::Quit, GamepadButtonType::East);
    // Insert MenuAction resources
    commands.insert_resource(input_map);
    commands.insert_resource(ActionState::<MenuAction>::default());
}

fn start_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            StateScoped(AppState::Menu),
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                style: Style { ..default() },
                text: Text::from_section(
                    "Kataster",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 100.0,
                        color: Color::srgb_u8(0x00, 0xAA, 0xAA),
                    },
                ),
                ..default()
            },));
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "enter",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 50.0,
                            color: Color::srgb_u8(0x00, 0x44, 0x44),
                        },
                    ),
                    ..default()
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
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            StateScoped(GameState::Over),
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                style: Style { ..default() },
                text: Text::from_section(
                    "Game Over",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 100.0,
                        color: Color::srgb_u8(0xAA, 0x22, 0x22),
                    },
                ),
                ..default()
            },));
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "enter",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 50.0,
                            color: Color::srgb_u8(0x88, 0x22, 0x22),
                        },
                    ),
                    ..default()
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
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            StateScoped(GameState::Paused),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "pause",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 100.0,
                            color: Color::srgb_u8(0xF8, 0xE4, 0x73),
                        },
                    ),
                    ..default()
                },
                DrawBlinkTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
            ));
        });
}

fn menu_blink_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DrawBlinkTimer, &ViewVisibility)>,
) {
    for (entity, mut timer, visibility) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            let new_visibility = if visibility.get() {
                Visibility::Hidden
            } else {
                Visibility::Visible
            };
            commands.entity(entity).insert(new_visibility);
        }
    }
}

fn menu_input_system(
    app_state: ResMut<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    menu_action_state: Res<ActionState<MenuAction>>,
    mut physics_time: ResMut<Time<Physics>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if app_state.get() != &AppState::Menu && menu_action_state.just_pressed(&MenuAction::ExitToMenu)
    {
        next_app_state.set(AppState::Menu);
        physics_time.unpause();
    } else if app_state.get() == &AppState::Menu {
        if menu_action_state.just_pressed(&MenuAction::Accept) {
            next_app_state.set(AppState::Game);
        }
        if menu_action_state.just_pressed(&MenuAction::Quit) {
            app_exit_events.send(AppExit::Success);
        }
    }
}

fn in_game_menu_input_system(
    game_state: ResMut<State<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    menu_action_state: Res<ActionState<MenuAction>>,
    mut physics_time: ResMut<Time<Physics>>,
) {
    match game_state.get() {
        GameState::Setup => {
            next_game_state.set(GameState::Running);
        }
        GameState::Running => {
            if menu_action_state.just_pressed(&MenuAction::PauseUnpause) {
                next_game_state.set(GameState::Paused);
                physics_time.pause();
            }
        }
        GameState::Paused => {
            if menu_action_state.just_pressed(&MenuAction::PauseUnpause) {
                next_game_state.set(GameState::Running);
                physics_time.unpause();
            }
        }
        GameState::Over => {
            if menu_action_state.just_pressed(&MenuAction::Accept) {
                next_app_state.set(AppState::Menu);
            }
        }
    }
}
