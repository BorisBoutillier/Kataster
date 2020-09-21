use super::components::*;
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
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    audio_output: Res<AudioOutput>,
) {
    let texture_handle = asset_server.load("assets/laserRed07.png").unwrap();
    let v = parent_body.position.rotation * Vector2::y() * 50.0;
    let body = RigidBodyBuilder::new_dynamic()
        .position(parent_body.position)
        .rotation(parent_body.position.rotation.angle())
        .linvel(v.x, v.y);
    let collider = ColliderBuilder::cuboid(0.25, 1.0).sensor(true);
    commands
        .spawn(SpriteComponents {
            transform: Transform::from_translation(Vec3::new(
                parent_body.position.translation.x,
                parent_body.position.translation.y,
                -4.0,
            ))
            .with_scale(1.0 / 18.0),
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with(Laser {
            despawn_timer: Timer::from_seconds(2.0, false),
        })
        .with(body)
        .with(collider);
    let sound = asset_server.load("assets/sfx_laser1.mp3").unwrap();
    audio_output.play(sound);
}

pub fn despawn_laser_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, Mut<Laser>)>,
) {
    for (entity, mut laser) in &mut query.iter() {
        laser.despawn_timer.tick(time.delta_seconds);
        if laser.despawn_timer.finished {
            commands.despawn(entity);
        }
    }
}
