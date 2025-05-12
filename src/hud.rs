use bevy::ecs::spawn::SpawnIter;

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
            Update,
            (hud_score_system, hud_life_system).run_if(in_state(GameState::Running)),
        )
        .add_systems(OnEnter(GameState::Setup), hud_spawn);
    }
}

fn hud_spawn(mut commands: Commands, assets: ResMut<UiAssets>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexEnd,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        StateScoped(AppState::Game),
        children![(
            Text::new("0"),
            TextFont {
                font: assets.font.clone(),
                font_size: 50.0,
                ..default()
            },
            TextColor(Color::srgb_u8(0x00, 0xAA, 0xAA)),
            TextLayout::new_with_justify(JustifyText::Right),
            Node {
                margin: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    top: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                },
                ..default()
            },
            UiScore {},
        )],
    ));
    // Life counters
    // Not kept in 'GameOver' state, simplifying last counter removal.
    let ship_life_image = assets.ship_life.clone();
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        StateScoped(AppState::Game),
        Children::spawn(SpawnIter((1..(START_LIFE + 1)).map(move |i| {
            (
                ImageNode::new(ship_life_image.clone()),
                Node {
                    margin: UiRect {
                        left: Val::Px(10.0),
                        right: Val::Px(10.0),
                        top: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                    },
                    ..default()
                },
                UiLife { min: i },
            )
        }))),
    ));
}

fn hud_score_system(arena: Res<Arena>, mut query: Query<&mut Text, With<UiScore>>) {
    if arena.is_changed() {
        for mut text in query.iter_mut() {
            **text = format!("{}", arena.score);
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
