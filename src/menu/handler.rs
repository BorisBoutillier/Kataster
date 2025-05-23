use bevy::ecs::spawn::SpawnIter;

use crate::prelude::*;

#[derive(Component)]
pub struct DrawBlink {
    pub timer: Timer,
    pub enabled: bool,
}

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
        let buttons = self
            .entries
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                (
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(45.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(10.0)),
                    ButtonId(i as i32),
                    children![(
                        Text::new(entry),
                        TextFont {
                            font: font.clone(),
                            font_size: 25.0,
                            ..default()
                        },
                        TextColor(self.main_text_color),
                    )],
                )
            })
            .collect::<Vec<_>>();
        let entity = commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                children![
                    (
                        Node {
                            height: Val::Percent(50.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        children![(
                            Text::new(self.main_text.clone()),
                            TextFont {
                                font: font.clone(),
                                font_size: 120.0,
                                ..default()
                            },
                            TextColor(self.main_text_color),
                            DrawBlink {
                                timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                                enabled: self.main_text_blink
                            }
                        )]
                    ),
                    (
                        Node {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        Children::spawn(SpawnIter(buttons.into_iter()))
                    )
                ],
            ))
            .insert(self)
            .id();
        entity
    }
}
pub fn menu_blink_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DrawBlink, &ViewVisibility)>,
) {
    for (entity, mut draw_blink, visibility) in query.iter_mut() {
        if draw_blink.enabled {
            draw_blink.timer.tick(time.delta());
            if draw_blink.timer.finished() {
                let new_visibility = if visibility.get() {
                    Visibility::Hidden
                } else {
                    Visibility::Visible
                };
                commands.entity(entity).insert(new_visibility);
            }
        }
    }
}

pub fn menu_selection_system(
    mut menu: Query<Mut<MenuHandler>>,
    menu_action_state: Res<ActionState<MenuAction>>,
    mut buttons: Query<(&ButtonId, &mut BorderColor, &mut BackgroundColor)>,
) {
    if let Ok(mut menu) = menu.single_mut() {
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
