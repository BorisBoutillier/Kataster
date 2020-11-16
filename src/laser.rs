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
    let v = parent_body.position.rotation * Vector2::y() * 50.0;
    let body = RigidBodyBuilder::new_dynamic()
        .position(parent_body.position)
        .rotation(parent_body.position.rotation.angle())
        .linvel(v.x, v.y);
    let collider = ColliderBuilder::cuboid(0.25, 1.0).sensor(true);
    commands
        .spawn(SpriteComponents {
            transform: Transform {
                translation: Vec3::new(
                    parent_body.position.translation.x,
                    parent_body.position.translation.y,
                    -4.0,
                ),
                scale: Vec3::splat(1.0 / 18.0),
                ..Default::default()
            },
            material: runstate.laser_texture_handle.clone(),
            ..Default::default()
        })
        .with(Laser {
            despawn_timer: Timer::from_seconds(2.0, false),
        })
        .with(body)
        .with(collider)
        .with(ForStates {
            states: vec![GameState::Game, GameState::Pause, GameState::GameOver],
        });
    audio.play(runstate.laser_audio_handle.clone());
}

pub fn despawn_laser_system(
    mut commands: Commands,
    runstate: Res<RunState>,
    time: Res<Time>,
    mut query: Query<(Entity, Mut<Laser>)>,
) {
    if runstate.gamestate.is(GameState::Game) {
        for (entity, mut laser) in query.iter_mut() {
            laser.despawn_timer.tick(time.delta_seconds);
            if laser.despawn_timer.finished {
                commands.despawn(entity);
            }
        }
    }
}
