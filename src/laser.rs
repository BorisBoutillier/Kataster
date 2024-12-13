use crate::prelude::*;

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
        app.add_event::<LaserSpawnEvent>().add_systems(
            Update,
            (laser_timeout_system, spawn_laser).run_if(in_state(GameState::Running)),
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
        let mut transform = spawn_event.transform;
        // Enforce laser sprite layer
        transform.translation.z = 2.0;
        let linvel = LinearVelocity(
            (spawn_event.linvel.0 * Vec2::Y) + (transform.rotation * Vec3::Y * 500.0).truncate(),
        );
        let collider = Collider::rectangle(2.5, 10.0);
        // It seems the way laser are spawned, xpbd does not create a ColliderMassProperties.
        // So I add it explicitly to avoid a runtime warning.
        // I did not search why the laser spawning is special.
        let mass_properties = MassPropertiesBundle::from_shape(&collider, 1.0);
        commands.spawn((
            Sprite {
                image: handles.laser.clone(),
                custom_size: Some(Vec2::new(5., 20.0)),
                ..default()
            },
            transform,
            Laser {
                despawn_timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
            CollisionLayers::new(GameLayer::Laser, [GameLayer::Asteroid]),
            RigidBody::Dynamic,
            collider,
            mass_properties,
            linvel,
            Sensor,
            AudioPlayer(audios.laser_trigger.clone()),
            StateScoped(AppState::Game),
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
