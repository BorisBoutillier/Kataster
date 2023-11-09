use crate::prelude::*;

#[derive(Event)]
pub struct LaserDespawnEvent(pub Entity);
#[derive(Event)]
pub struct LaserSpawnEvent {
    // The full position (translation+rotation) of the laser to spawn
    pub transform: Transform,
    // The velocity of the entity emitting the laser
    pub linvel: LinearVelocity,
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
            .add_systems(
                Update,
                (laser_timeout_system, spawn_laser).run_if(in_state(AppState::GameRunning)),
            );
    }
}

fn spawn_laser(
    mut commands: Commands,
    mut laser_spawn_events: EventReader<LaserSpawnEvent>,
    handles: Res<SpriteAssets>,
    audios: Res<AudioAssets>,
) {
    for spawn_event in laser_spawn_events.read() {
        let transform = spawn_event.transform;
        let position = Position(spawn_event.transform.translation.truncate());
        let rotation: Rotation = transform.rotation.into();
        let linvel = LinearVelocity(
            (spawn_event.linvel.0 * Vec2::Y) + (transform.rotation * Vec3::Y * 500.0).truncate(),
        );
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(5., 20.0)),
                    ..default()
                },
                // Transform Z is meaningfull for sprite stacking.
                // Transform X,Y and rotation will be computed from xpbd Position and Rotation components
                transform: Transform {
                    translation: Vec3::Z * 2.0,
                    ..default()
                },
                texture: handles.laser.clone(),
                ..default()
            },
            Laser {
                despawn_timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
            ForState {
                states: AppState::ANY_GAME_STATE.to_vec(),
            },
            RigidBody::Dynamic,
            Collider::cuboid(2.5, 10.0),
            position,
            rotation,
            linvel,
            Sensor,
            AudioBundle {
                source: audios.laser_trigger.clone(),
                ..default()
            },
        ));
    }
}

fn laser_timeout_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Laser)>,
) {
    for (entity, mut laser) in query.iter_mut() {
        laser.despawn_timer.tick(time.delta());
        if laser.despawn_timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
