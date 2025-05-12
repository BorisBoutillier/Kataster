use crate::prelude::*;
use core::time::Duration;

#[derive(Event)]
pub struct AsteroidSpawnEvent {
    pub size: AsteroidSize,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub angvel: f32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AsteroidSize {
    Big,
    Medium,
    Small,
}
impl std::fmt::Display for AsteroidSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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

    // Defines for each if the asteroid is split on destruction
    // And the spawned sub-asteroid size and radius of spawning.
    pub fn split(&self) -> Option<(AsteroidSize, f32)> {
        match self {
            AsteroidSize::Big => Some((AsteroidSize::Medium, 20.0)),
            AsteroidSize::Medium => Some((AsteroidSize::Small, 10.0)),
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
        app.add_event::<AsteroidSpawnEvent>().add_systems(
            Update,
            (arena_asteroids, spawn_asteroid_event).run_if(in_state(GameState::Running)),
        );
    }
}

fn spawn_asteroid_event(
    mut commands: Commands,
    mut event_reader: EventReader<AsteroidSpawnEvent>,
    handles: Res<SpriteAssets>,
) {
    for event in event_reader.read() {
        let (sprite_handle, radius) = match event.size {
            AsteroidSize::Big => (handles.meteor_big.clone(), 101. / 2.0),
            AsteroidSize::Medium => (handles.meteor_med.clone(), 43. / 2.0),
            AsteroidSize::Small => (handles.meteor_small.clone(), 28. / 2.0),
        };
        commands
            .spawn((
                Name::new(format!("Asteroid {}", event.size)),
                Sprite {
                    image: sprite_handle.clone(),
                    ..default()
                },
                Transform::from_translation(Vec3::new(event.x, event.y, 1.0)),
                Asteroid { size: event.size },
                StateScoped(AppState::Game),
                CollisionLayers::new(
                    GameLayer::Asteroid,
                    [GameLayer::Asteroid, GameLayer::Player, GameLayer::Laser],
                ),
                RigidBody::Dynamic,
                Collider::circle(radius),
                Restitution::new(0.5),
                LinearVelocity(Vec2::new(event.vx, event.vy)),
                AngularVelocity(event.angvel),
            ))
            .observe(on_asteroid_damage);
    }
}

fn arena_asteroids(
    time: Res<Time>,
    mut arena: ResMut<Arena>,
    mut asteroid_spawn_events: EventWriter<AsteroidSpawnEvent>,
    asteroids: Query<&Asteroid>,
) {
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
            asteroid_spawn_events.write(AsteroidSpawnEvent {
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

fn on_asteroid_damage(
    trigger: Trigger<Damage>,
    mut commands: Commands,
    mut arena: ResMut<Arena>,
    mut asteroid_spawn_events: EventWriter<AsteroidSpawnEvent>,
    asteroids: Query<(&Asteroid, &Transform, &AngularVelocity)>,
) {
    let asteroid_entity = trigger.target();
    let (asteroid, asteroid_transform, asteroid_angvel) = asteroids.get(asteroid_entity).unwrap();
    arena.score += asteroid.size.score();
    {
        if let Some((size, radius)) = asteroid.size.split() {
            let mut rng = thread_rng();
            for i in 0..4 {
                //rng.gen_range(1..4u8) {
                let x_pos = if i % 2 == 0 { 1. } else { -1. };
                let y_pos = if (i / 2) % 2 == 0 { 1. } else { -1. };
                let x = asteroid_transform.translation.x + x_pos * 1.5 * radius;
                let y = asteroid_transform.translation.y + y_pos * 1.5 * radius;
                let vx =
                    rng.gen_range((-ARENA_WIDTH / (radius / 4.))..(ARENA_WIDTH / (radius / 4.)));
                let vy =
                    rng.gen_range((-ARENA_HEIGHT / (radius / 4.))..(ARENA_HEIGHT / (radius / 4.)));
                asteroid_spawn_events.write(AsteroidSpawnEvent {
                    size,
                    x,
                    y,
                    vx,
                    vy,
                    angvel: asteroid_angvel.0,
                });
            }
        }
    }
    commands.entity(asteroid_entity).despawn();
}
