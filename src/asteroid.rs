use crate::prelude::*;
use bevy::utils::Duration;

pub struct AsteroidSpawnEvent {
    pub size: AsteroidSize,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub angvel: f32,
}

pub struct LaserAsteroidContactEvent {
    pub laser: Entity,
    pub asteroid: Entity,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AsteroidSize {
    Big,
    Medium,
    Small,
}
impl AsteroidSize {
    // Score marked when destroying an asteroid of this size
    pub fn score(&self) -> u32 {
        match self {
            AsteroidSize::Big => 40,
            AsteroidSize::Medium => 20,
            AsteroidSize::Small => 10,
        }
    }

    // Defines for each if the asteroid is splitted on destruction
    // And the spawned sub-asteroid size and radius of spawning.
    pub fn split(&self) -> Option<(AsteroidSize, f32)> {
        match self {
            AsteroidSize::Big => Some((AsteroidSize::Medium, 10.0)),
            AsteroidSize::Medium => Some((AsteroidSize::Small, 5.0)),
            AsteroidSize::Small => None,
        }
    }
}
#[derive(Component)]
pub struct Asteroid {
    pub size: AsteroidSize,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AsteroidSpawnEvent>()
            .add_event::<LaserAsteroidContactEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(arena_asteroids)
                    .with_system(spawn_asteroid_event)
                    .with_system(asteroid_damage.after(ContactLabel)),
            );
    }
}

fn spawn_asteroid_event(
    mut commands: Commands,
    mut event_reader: EventReader<AsteroidSpawnEvent>,
    handles: Res<SpriteAssets>,
) {
    for event in event_reader.iter() {
        let (sprite_handle, radius) = match event.size {
            AsteroidSize::Big => (handles.meteor_big.clone(), 101. / 2.0),
            AsteroidSize::Medium => (handles.meteor_med.clone(), 43. / 2.0),
            AsteroidSize::Small => (handles.meteor_small.clone(), 28. / 2.0),
        };
        commands.spawn((
            SpriteBundle {
                // No custom size, the sprite png, are already at our game size.
                transform: Transform {
                    translation: Vec3::new(event.x, event.y, 1.0),
                    ..Default::default()
                },
                texture: sprite_handle.clone(),
                ..Default::default()
            },
            Asteroid { size: event.size },
            Damage { value: 1 },
            ForState {
                states: vec![AppState::Game],
            },
            RigidBody::Dynamic,
            Collider::ball(radius),
            ActiveEvents::COLLISION_EVENTS,
            Velocity {
                linvel: Vec2::new(event.vx, event.vy),
                angvel: event.angvel,
            },
        ));
    }
}

fn arena_asteroids(
    time: Res<Time>,
    gamestate: Res<State<AppGameState>>,
    mut arena: ResMut<Arena>,
    mut asteroid_spawn_events: EventWriter<AsteroidSpawnEvent>,
    asteroids: Query<&Asteroid>,
) {
    if gamestate.current() == &AppGameState::Game {
        arena.asteroid_spawn_timer.tick(time.delta());
        if arena.asteroid_spawn_timer.finished() {
            arena.asteroid_spawn_timer.reset();
            let n_asteroid = asteroids.iter().count();
            if n_asteroid < 20 {
                let duration = Duration::from_secs_f32(
                    (0.8 * arena.asteroid_spawn_timer.duration().as_secs_f32()).max(0.1),
                );
                arena.asteroid_spawn_timer.set_duration(duration);
                let mut rng = thread_rng();
                // 0: Top , 1:Left
                let side = rng.gen_range(0..2u8);
                let (x, y) = match side {
                    0 => (
                        rng.gen_range((-ARENA_WIDTH / 2.0)..(ARENA_WIDTH / 2.0)),
                        ARENA_HEIGHT / 2.0,
                    ),
                    _ => (
                        -ARENA_WIDTH / 2.0,
                        rng.gen_range((-ARENA_HEIGHT / 2.0)..(ARENA_HEIGHT / 2.0)),
                    ),
                };
                let vx = rng.gen_range((-ARENA_WIDTH / 4.0)..(ARENA_WIDTH / 4.0));
                let vy = rng.gen_range((-ARENA_HEIGHT / 4.0)..(ARENA_HEIGHT / 4.0));
                let angvel = rng.gen_range(-10.0..10.0);
                asteroid_spawn_events.send(AsteroidSpawnEvent {
                    size: AsteroidSize::Big,
                    x,
                    y,
                    vx,
                    vy,
                    angvel,
                });
            }
        }
    }
}

fn asteroid_damage(
    mut commands: Commands,
    mut arena: ResMut<Arena>,
    mut laser_asteroid_contact_events: EventReader<LaserAsteroidContactEvent>,
    mut explosion_spawn_events: EventWriter<SpawnExplosionEvent>,
    mut asteroid_spawn_events: EventWriter<AsteroidSpawnEvent>,
    transforms: Query<&Transform>,
    asteroids: Query<(&Asteroid, &Transform, &Velocity)>,
) {
    for event in laser_asteroid_contact_events.iter() {
        let laser_transform = transforms.get(event.laser).unwrap();
        let (asteroid, asteroid_transform, asteroid_velocity) =
            asteroids.get(event.asteroid).unwrap();
        arena.score += asteroid.size.score();
        {
            explosion_spawn_events.send(SpawnExplosionEvent {
                kind: ExplosionKind::LaserOnAsteroid,
                x: laser_transform.translation.x,
                y: laser_transform.translation.y,
            });
            if let Some((size, radius)) = asteroid.size.split() {
                let mut rng = thread_rng();
                for _ in 0..rng.gen_range(1..4u8) {
                    let x = asteroid_transform.translation.x + rng.gen_range(-radius..radius);
                    let y = asteroid_transform.translation.y + rng.gen_range(-radius..radius);
                    let vx = rng.gen_range((-ARENA_WIDTH / radius)..(ARENA_WIDTH / radius));
                    let vy = rng.gen_range((-ARENA_HEIGHT / radius)..(ARENA_HEIGHT / radius));
                    asteroid_spawn_events.send(AsteroidSpawnEvent {
                        size,
                        x,
                        y,
                        vx,
                        vy,
                        angvel: asteroid_velocity.angvel,
                    });
                }
            }
        }
        commands.entity(event.laser).despawn();
        commands.entity(event.asteroid).despawn();
    }
}
