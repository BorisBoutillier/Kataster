use crate::prelude::*;

pub struct LaserDespawnEvent(pub Entity);
pub struct LaserSpawnEvent {
    // The full position (translation+rotation) of the laser to spawn
    pub transform: Transform,
    // The velocity of the entity emitting the laser
    pub velocity: Velocity,
}

#[derive(Component)]
pub struct Laser {
    pub despawn_timer: Timer,
}
pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LaserDespawnEvent>()
            .add_event::<LaserSpawnEvent>()
            .add_systems((laser_timeout_system, spawn_laser).in_set(OnUpdate(AppState::Game)));
    }
}

fn spawn_laser(
    mut commands: Commands,
    mut laser_spawn_events: EventReader<LaserSpawnEvent>,
    handles: Res<SpriteAssets>,
    audios: Res<AudioAssets>,
    audio_output: Res<Audio>,
) {
    for spawn_event in laser_spawn_events.iter() {
        let transform = spawn_event.transform;
        let velocity = Velocity::linear(
            (spawn_event.velocity.linvel * Vec2::Y)
                + (transform.rotation * Vec3::Y * 500.0).truncate(),
        );
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(5., 20.0)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(transform.translation.x, transform.translation.y, 2.0),
                    rotation: transform.rotation,
                    ..Default::default()
                },
                texture: handles.laser.clone(),
                ..Default::default()
            },
            Laser {
                despawn_timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
            ForState {
                states: vec![AppState::Game],
            },
            RigidBody::Dynamic,
            Collider::cuboid(2.5, 10.0),
            velocity,
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
        ));
        audio_output.play(audios.laser_trigger.clone());
    }
}

fn laser_timeout_system(
    mut commands: Commands,
    time: Res<Time>,
    gamestate: Res<State<AppGameState>>,
    mut query: Query<(Entity, &mut Laser)>,
) {
    if gamestate.0 == AppGameState::Game {
        for (entity, mut laser) in query.iter_mut() {
            laser.despawn_timer.tick(time.delta());
            if laser.despawn_timer.finished() {
                commands.entity(entity).despawn();
            }
        }
    }
}
