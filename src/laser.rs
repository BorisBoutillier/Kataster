use crate::prelude::*;

#[derive(Message)]
pub struct LaserSpawnMessage {
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
        app.add_message::<LaserSpawnMessage>().add_systems(
            Update,
            (spawn_laser, laser_asteroid_collision, laser_timeout_system)
                .run_if(in_state(GameState::Running)),
        );
    }
}

fn spawn_laser(
    mut commands: Commands,
    mut laser_spawn_events: MessageReader<LaserSpawnMessage>,
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
            Name::new("Laser"),
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
            CollidingEntities::default(),
            RigidBody::Dynamic,
            collider,
            mass_properties,
            linvel,
            Sensor,
            AudioPlayer(audios.laser_trigger.clone()),
            DespawnOnExit(AppState::Game),
        ));
    }
}

fn laser_asteroid_collision(
    mut commands: Commands,
    mut explosion_spawn_events: MessageWriter<SpawnExplosionMessage>,
    laser_collisions: Query<(Entity, &CollidingEntities), With<Laser>>,
    is_asteroid: Query<(), With<Asteroid>>,
    transforms: Query<&Transform>,
) {
    for (laser, targets) in laser_collisions.iter() {
        for target in targets.iter() {
            // Laser on Asteroid collision
            // The asteroid is damaged and the laser despawned.
            // A LaserOnAsteroid explosion VFX is triggered. To simplify code
            // the VFX is triggered at the laser position and not at the exact contact position.
            if is_asteroid.contains(*target) {
                commands.trigger(Damage { entity: *target });
                let laser_transform = transforms
                    .get(laser)
                    .expect("Missing transform for the laser");
                explosion_spawn_events.write(SpawnExplosionMessage {
                    kind: ExplosionKind::LaserOnAsteroid,
                    x: laser_transform.translation.x,
                    y: laser_transform.translation.y,
                });
                commands.entity(laser).despawn();
            }
        }
    }
}

fn laser_timeout_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Laser)>,
) {
    for (entity, mut laser) in query.iter_mut() {
        laser.despawn_timer.tick(time.delta());
        if laser.despawn_timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
