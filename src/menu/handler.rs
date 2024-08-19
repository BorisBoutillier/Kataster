use crate::prelude::*;

#[derive(Component)]
pub struct DrawBlinkTimer(pub Timer);

#[derive(Component)]
pub struct ButtonId(i32);

#[derive(Component)]
pub struct MenuHandler {
    pub main_text: String,
    pub main_text_color: Color,
    pub main_text_blink: bool,
    pub entries: Vec<String>,
    pub selected_id: i32,
}
impl MenuHandler {
    const SELECTED_BORDER: Color = Color::srgb(0.4, 0.4, 0.4);
    const SELECTED_BG: Color = Color::srgb(0.2, 0.2, 0.2);
    const UNSELECTED_BORDER: Color = Color::srgb(0.2, 0.2, 0.2);
    const UNSELECTED_BG: Color = Color::srgb(0.0, 0.0, 0.0);
    pub fn spawn(self, commands: &mut Commands, font: Handle<Font>) -> Entity {
        let button_style = Style {
            width: Val::Px(150.0),
            height: Val::Px(45.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
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
            .insert(self)
            .id();
        entity
    }
}
pub fn menu_blink_system(
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

pub fn menu_selection_system(
    mut menu: Query<Mut<MenuHandler>>,
    menu_action_state: Res<ActionState<MenuAction>>,
    mut buttons: Query<(&ButtonId, &mut BorderColor, &mut BackgroundColor)>,
) {
    if let Ok(mut menu) = menu.get_single_mut() {
        if menu_action_state.just_pressed(&MenuAction::MenuUp) {
            menu.selected_id = (menu.selected_id - 1).rem_euclid(menu.entries.len() as i32);
        }
        if menu_action_state.just_pressed(&MenuAction::MenuDown) {
            menu.selected_id = (menu.selected_id + 1).rem_euclid(menu.entries.len() as i32);
        }
        if menu.is_changed() {
            for (button_id, mut border_color, mut bg_color) in buttons.iter_mut() {
                if button_id.0 == menu.selected_id {
                    border_color.0 = MenuHandler::SELECTED_BORDER;
                    bg_color.0 = MenuHandler::SELECTED_BG;
                } else {
                    border_color.0 = MenuHandler::UNSELECTED_BORDER;
                    bg_color.0 = MenuHandler::UNSELECTED_BG;
                }
            }
        }
    }
}
