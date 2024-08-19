use bevy::app::AppExit;

use crate::prelude::*;

#[derive(Component)]
pub struct DrawBlinkTimer(pub Timer);

#[derive(Component)]
pub struct ButtonId(i32);

#[derive(Resource, Default)]
struct Menu {
    main_text: String,
    main_text_color: Color,
    main_text_blink: bool,
    entries: Vec<String>,
    selected_id: i32,
}
impl Menu {
    const SELECTED_BORDER: Color = Color::srgb(0.4, 0.4, 0.4);
    const SELECTED_BG: Color = Color::srgb(0.2, 0.2, 0.2);
    const UNSELECTED_BORDER: Color = Color::srgb(0.2, 0.2, 0.2);
    const UNSELECTED_BG: Color = Color::srgb(0.0, 0.0, 0.0);
    fn spawn(self, commands: &mut Commands, font: Handle<Font>) -> Entity {
        let button_style = Style {
            width: Val::Px(150.0),
            height: Val::Px(45.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        };
        let entity = commands
            .spawn((NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },))
            .with_children(|parent| {
                parent
                    .spawn((NodeBundle {
                        style: Style {
                            height: Val::Percent(50.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },))
                    .with_children(|parent| {
                        let mut text = parent.spawn((TextBundle {
                            style: Style { ..default() },
                            text: Text::from_section(
                                self.main_text.clone(),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 120.0,
                                    color: self.main_text_color,
                                },
                            ),
                            ..default()
                        },));
                        if self.main_text_blink {
                            text.insert(DrawBlinkTimer(Timer::from_seconds(
                                0.5,
                                TimerMode::Repeating,
                            )));
                        }
                    });
                parent
                    .spawn((NodeBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    },))
                    .with_children(|parent| {
                        for (i, entry) in self.entries.iter().enumerate() {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: button_style.clone(),
                                        border_radius: BorderRadius::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    ButtonId(i as i32),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        entry,
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 25.0,
                                            color: self.main_text_color,
                                        },
                                    ));
                                });
                        }
                    });
            })
            .id();
        commands.insert_resource(self);
        entity
    }
}
// List of user actions associated to menu/ui interaction
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum MenuAction {
    // In menus move up the highlighted entry
    MenuUp,
    // In menus move down the highlighted entry
    MenuDown,
    // In menus, select highlighted entry
    Accept,
    // During gameplay, pause the game.
    // Also directly unpause the game when in the pause screen.
    PauseUnpause,
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Menu::default())
            .add_systems(OnEnter(AppState::Setup), setup)
            .add_systems(OnEnter(AppState::Menu), spawn_main_menu)
            .add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
            .add_systems(OnEnter(GameState::Over), spawn_gameover_menu)
            .add_systems(
                Update,
                (
                    main_menu_input_system,
                    menu_selection_system,
                    menu_blink_system,
                ),
            )
            .add_systems(
                Update,
                game_menu_input_system.run_if(in_state(AppState::Game)),
            );
    }
}

const MAINMENU_TEXT_COLOR: Color = Color::srgb(0.0, 0.7, 0.7);

fn setup(mut commands: Commands) {
    let mut input_map = InputMap::<MenuAction>::new([
        (MenuAction::Accept, KeyCode::Enter),
        (MenuAction::PauseUnpause, KeyCode::Escape),
        (MenuAction::MenuUp, KeyCode::KeyW),
        (MenuAction::MenuUp, KeyCode::ArrowUp),
        (MenuAction::MenuDown, KeyCode::KeyS),
        (MenuAction::MenuDown, KeyCode::ArrowDown),
    ]);
    input_map.insert(MenuAction::PauseUnpause, GamepadButtonType::Start);
    input_map.insert(MenuAction::Accept, GamepadButtonType::South);
    // Insert MenuAction resources
    commands.insert_resource(input_map);
    commands.insert_resource(ActionState::<MenuAction>::default());
}

fn spawn_main_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
    let entity = Menu {
        main_text: "Kataster".into(),
        main_text_color: MAINMENU_TEXT_COLOR,
        main_text_blink: false,
        selected_id: 0,
        entries: vec!["Play".into(), "Credits".into(), "Exit".into()],
    }
    .spawn(&mut commands, assets.font.clone());
    commands.entity(entity).insert(StateScoped(AppState::Menu));
}

fn spawn_gameover_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
    let entity = Menu {
        main_text: "Game Over".into(),
        main_text_color: Color::srgb_u8(0xAA, 0x22, 0x22),
        main_text_blink: false,
        selected_id: 0,
        entries: vec!["Menu".into(), "Exit".into()],
    }
    .spawn(&mut commands, assets.font.clone());
    commands.entity(entity).insert(StateScoped(GameState::Over));
}

fn spawn_pause_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
    let entity = Menu {
        main_text: "Pause".into(),
        main_text_color: Color::srgb_u8(0xF8, 0xE4, 0x73),
        main_text_blink: true,
        selected_id: 0,
        entries: vec!["Resume".into(), "Menu".into(), "Exit".into()],
    }
    .spawn(&mut commands, assets.font.clone());
    commands
        .entity(entity)
        .insert(StateScoped(GameState::Paused));
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

fn menu_selection_system(
    mut menu_selection: ResMut<Menu>,
    menu_action_state: Res<ActionState<MenuAction>>,
    mut buttons: Query<(&ButtonId, &mut BorderColor, &mut BackgroundColor)>,
) {
    if menu_action_state.just_pressed(&MenuAction::MenuUp) {
        menu_selection.selected_id =
            (menu_selection.selected_id - 1).rem_euclid(menu_selection.entries.len() as i32);
    }
    if menu_action_state.just_pressed(&MenuAction::MenuDown) {
        menu_selection.selected_id =
            (menu_selection.selected_id + 1).rem_euclid(menu_selection.entries.len() as i32);
    }
    if menu_selection.is_changed() {
        for (button_id, mut border_color, mut bg_color) in buttons.iter_mut() {
            if button_id.0 == menu_selection.selected_id {
                border_color.0 = Menu::SELECTED_BORDER;
                bg_color.0 = Menu::SELECTED_BG;
            } else {
                border_color.0 = Menu::UNSELECTED_BORDER;
                bg_color.0 = Menu::UNSELECTED_BG;
            }
        }
    }
}

fn main_menu_input_system(
    app_state: ResMut<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    menu_action_state: Res<ActionState<MenuAction>>,
    mut app_exit_events: EventWriter<AppExit>,
    menu_selection: Res<Menu>,
) {
    if app_state.get() == &AppState::Menu && menu_action_state.just_pressed(&MenuAction::Accept) {
        match menu_selection.selected_id {
            0 => {
                next_app_state.set(AppState::Game);
            }
            _ => {
                app_exit_events.send(AppExit::Success);
            }
        }
    }
}
fn game_menu_input_system(
    game_state: ResMut<State<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    menu_action_state: Res<ActionState<MenuAction>>,
    mut app_exit_events: EventWriter<AppExit>,
    menu_selection: Res<Menu>,
) {
    if menu_action_state.just_pressed(&MenuAction::PauseUnpause) {
        if game_state.get() == &GameState::Running {
            next_game_state.set(GameState::Paused);
        }
        if game_state.get() == &GameState::Paused {
            next_game_state.set(GameState::Running);
        }
    }
    if menu_action_state.just_pressed(&MenuAction::Accept) {
        if game_state.get() == &GameState::Paused {
            match menu_selection.selected_id {
                0 => {
                    next_game_state.set(GameState::Running);
                }
                1 => {
                    next_app_state.set(AppState::Menu);
                }
                _ => {
                    app_exit_events.send(AppExit::Success);
                }
            }
        }
        if game_state.get() == &GameState::Over {
            match menu_selection.selected_id {
                0 => {
                    next_app_state.set(AppState::Menu);
                }
                _ => {
                    app_exit_events.send(AppExit::Success);
                }
            }
        }
    }
}
