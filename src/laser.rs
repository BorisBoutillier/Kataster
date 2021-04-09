use super::components::*;
use super::state::*;
use bevy::prelude::*;
use bevy_rapier2d::{
    na::Vector2,
    rapier::{
        dynamics::{RigidBody, RigidBodyBuilder},
        geometry::ColliderBuilder,
        //        math::Point,
    },
};

pub fn spawn_laser(
    mut commands: Commands,
    parent_body: &RigidBody,
    runstate: &RunState,
    audio: Res<Audio>,
) {
    let v = parent_body.position().rotation * Vector2::y() * 50.0;
    let mut entity_builder = commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(
                parent_body.position().translation.x,
                parent_body.position().translation.y,
                -4.0,
            ),
            scale: Vec3::splat(1.0 / 18.0),
            ..Default::default()
        },
        material: runstate.laser_texture_handle.clone(),
        ..Default::default()
    });
    entity_builder
        .insert(Laser {
            despawn_timer: Timer::from_seconds(2.0, false),
        })
        .insert(ForState {
            states: vec![AppState::Game],
        });
    let body = RigidBodyBuilder::new_dynamic()
        .position(parent_body.position().clone())
        .rotation(parent_body.position().rotation.angle())
        .linvel(v.x, v.y)
        .user_data(entity_builder.id().to_bits() as u128);
    let collider = ColliderBuilder::cuboid(0.25, 1.0).sensor(true);
    entity_builder.insert_bundle((body, collider));
    audio.play(runstate.laser_audio_handle.clone());
}

pub fn despawn_laser_system(
    mut commands: Commands,
    gamestate: Res<State<AppGameState>>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Laser)>,
) {
    if gamestate.current() == &AppGameState::Game {
        for (entity, mut laser) in query.iter_mut() {
            laser.despawn_timer.tick(time.delta());
            if laser.despawn_timer.finished() {
                commands.entity(entity).despawn();
            }
        }
    }
}
