use crate::prelude::*;

pub struct LaserDespawnEvent(pub Entity);
pub struct LaserSpawnEvent {
    // The full position (translation+rotation) of the laser to spawn
    pub transform: Transform,
    // The velocity of the entity emitting the laser
    pub velocity: Velocity,
}

#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct CanDespawnLaserLabel;
#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct CanSpawnLaserLabel;

#[derive(Component)]
pub struct Laser {
    pub despawn_timer: Timer,
}
pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LaserDespawnEvent>()
            .add_event::<LaserSpawnEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(laser_timeout_system.label(CanDespawnLaserLabel))
                    .with_system(spawn_laser.after(CanSpawnLaserLabel))
                    .with_system(despawn_laser_system.after(CanDespawnLaserLabel)),
            );
    }
}

pub fn spawn_laser(
    mut commands: Commands,
    mut laser_spawn_events: EventReader<LaserSpawnEvent>,
    runstate: Res<RunState>,
    audio: Res<Audio>,
) {
    for spawn_event in laser_spawn_events.iter() {
        let transform = spawn_event.transform;
        let velocity = Velocity::linear(
            (spawn_event.velocity.linvel * Vec2::Y)
                + (transform.rotation * Vec3::Y * 500.0).truncate(),
        );
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(5., 20.0)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(transform.translation.x, transform.translation.y, -4.0),
                    rotation: transform.rotation,
                    ..Default::default()
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
            .insert(Collider::cuboid(2.5, 10.0))
            .insert(velocity)
            .insert(Sensor)
            .insert(ActiveEvents::COLLISION_EVENTS);
        audio.play(runstate.laser_audio_handle.clone());
    }
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
