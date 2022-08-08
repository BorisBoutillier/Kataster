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
        .insert(Collider::cuboid(0.25 * 10.0, 1.0 * 10.0))
        .insert(Velocity::linear(Vec2::new(v.x, v.y)))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS);
    audio.play(runstate.laser_audio_handle.clone());
}

pub fn laser_timeout_system(
    gamestate: Res<State<AppGameState>>,
    mut laser_despawn_events: EventWriter<LaserDespawnEvent>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Laser)>,
) {
    if gamestate.current() == &AppGameState::Game {
        for (entity, mut laser) in query.iter_mut() {
            laser.despawn_timer.tick(time.delta());
            if laser.despawn_timer.finished() {
                laser_despawn_events.send(LaserDespawnEvent(entity));
            }
        }
    }
}

pub fn despawn_laser_system(
    mut commands: Commands,
    mut event_reader: EventReader<LaserDespawnEvent>,
) {
    for event in event_reader.iter() {
        commands.entity(event.0).despawn();
    }
}
