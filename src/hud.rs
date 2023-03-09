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
        app.add_systems(
            (hud_score_system, hud_life_system).in_set(OnUpdate(AppState::GameRunning)),
        )
        .add_system(hud_spawn.in_schedule(OnEnter(AppState::GameCreate)));
    }
}

fn hud_spawn(mut commands: Commands, assets: ResMut<UiAssets>) {
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
                states: AppState::ANY_GAME_STATE.to_vec(),
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
                            font: assets.font.clone(),
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
                states: AppState::ANY_GAME_STATE.to_vec(),
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
                        image: assets.ship_life.clone(),
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
fn hud_life_system(
    mut commands: Commands,
    ship_query: Query<&Ship, Changed<Ship>>,
    mut uilife_query: Query<(Entity, &UiLife)>,
) {
    let mut life = 0;
    for ship in ship_query.iter() {
        if ship.player_id == 1 {
            life = ship.life;
        }
    }
    for (entity, uilife) in uilife_query.iter_mut() {
        commands.entity(entity).insert(if life >= uilife.min {
            Visibility::Visible
        } else {
            Visibility::Hidden
        });
    }
}
