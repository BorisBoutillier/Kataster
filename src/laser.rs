use crate::prelude::*;

pub fn spawn_laser(
    mut commands: Commands,
    transform: &Transform,
    runstate: &RunState,
    audio: Res<Audio>,
) {
    let v = transform.rotation * Vec3::Y * 50.0;
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(transform.translation.x, transform.translation.y, -4.0),
                rotation: transform.rotation,
                scale: Vec3::splat(1.0 / 18.0),
            },
            texture: runstate.laser_texture_handle.clone(),
            ..Default::default()
        })
        .insert(Laser {
            despawn_timer: Timer::from_seconds(2.0, false),
        })
        .insert(ForState {
            states: vec![AppState::Game],
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(0.25, 1.0, 0.0),
            border_radius: None,
        })
        .insert(Velocity::from_linear(Vec3::new(v.x, v.y, 0.0)))
        .insert(
            CollisionLayers::none()
                .with_group(ArenaLayer::Laser)
                .with_mask(ArenaLayer::World),
        );
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
