use crate::prelude::*;

#[derive(Component)]
pub struct UiScore {}
#[derive(Component)]
pub struct UiLife {
    pub min: u32,
}

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(hud_score_system)
                .with_system(hud_life_system),
        )
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(hud_spawn));
    }
}

fn hud_spawn(mut commands: Commands, handles: ResMut<GameAssets>, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexEnd,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                ..Default::default()
            },
            ForState {
                states: vec![AppState::Game],
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
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
                            font: handles.font.clone(),
                            font_size: 50.0,
                            color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                        },
                    ),
                    ..Default::default()
                },
                UiScore {},
            ));
        });
    // Life counters
    // Not kept in 'GameOver' state, simplifying last counter removal.
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                ..Default::default()
            },
            ForState {
                states: vec![AppState::Game],
            },
        ))
        .with_children(|parent| {
            for i in 1..(START_LIFE + 1) {
                parent.spawn((
                    ImageBundle {
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
                    },
                    UiLife { min: i },
                ));
            }
        });
}

fn hud_score_system(arena: Res<Arena>, mut query: Query<&mut Text, With<UiScore>>) {
    if arena.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{}", arena.score);
        }
    }
}
fn hud_life_system(ship_query: Query<&Ship>, mut uilife_query: Query<(&mut Visibility, &UiLife)>) {
    let mut life = 0;
    for ship in ship_query.iter() {
        if ship.player_id == 1 {
            life = ship.life;
        }
    }
    for (mut visibility, uilife) in uilife_query.iter_mut() {
        visibility.is_visible = life >= uilife.min;
    }
}
