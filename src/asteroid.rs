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
        app.add_event::<AsteroidSpawnEvent>().add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(arena_asteroids)
                .with_system(spawn_asteroid_event),
        );
    }
}

pub fn spawn_asteroid_event(
    mut commands: Commands,
    mut event_reader: EventReader<AsteroidSpawnEvent>,
    runstate: Res<RunState>,
) {
    for event in event_reader.iter() {
        let (sprite_handle, radius) = match event.size {
            AsteroidSize::Big => (runstate.meteor_big_handle.clone(), 101. / 2.0),
            AsteroidSize::Medium => (runstate.meteor_med_handle.clone(), 43. / 2.0),
            AsteroidSize::Small => (runstate.meteor_small_handle.clone(), 28. / 2.0),
        };
        commands
            .spawn_bundle(SpriteBundle {
                // No custom size, the sprite png, are already at our game size.
                transform: Transform {
                    translation: Vec3::new(event.x, event.y, -5.0),
                    ..Default::default()
                },
                texture: sprite_handle.clone(),
                ..Default::default()
            })
            .insert(Asteroid { size: event.size })
            .insert(Damage { value: 1 })
            .insert(ForState {
                states: vec![AppState::Game],
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(radius))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Velocity {
                linvel: Vec2::new(event.vx, event.vy),
                angvel: event.angvel,
            });
    }
}

pub fn arena_asteroids(
    time: Res<Time>,
    gamestate: Res<State<AppGameState>>,
    mut runstate: ResMut<RunState>,
    mut asteroid_spawn_events: EventWriter<AsteroidSpawnEvent>,
    asteroids: Query<&Asteroid>,
) {
    if gamestate.current() == &AppGameState::Game {
        let arena = runstate.arena.as_mut().unwrap();
        arena.asteroid_spawn_timer.tick(time.delta());
        if arena.asteroid_spawn_timer.finished() {
            arena.asteroid_spawn_timer.reset();
            let n_asteroid = asteroids.iter().count();
            if n_asteroid < 20 {
                arena
                    .asteroid_spawn_timer
                    .set_duration(Duration::from_secs_f32(
                        (0.8 * arena.asteroid_spawn_timer.duration().as_secs_f32()).max(0.1),
                    ));
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
