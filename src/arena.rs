use crate::prelude::*;
use bevy::utils::Duration;
use bevy_rapier2d::physics::RigidBodyHandleComponent;
use bevy_rapier2d::rapier::{
    dynamics::{RigidBodyBuilder, RigidBodySet},
    geometry::ColliderBuilder,
    //        math::Point,
};
use rand::{thread_rng, Rng};

pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 800;
pub const CAMERA_SCALE: f32 = 0.1;
pub const ARENA_WIDTH: f32 = WINDOW_WIDTH as f32 * CAMERA_SCALE;
pub const ARENA_HEIGHT: f32 = WINDOW_HEIGHT as f32 * CAMERA_SCALE;

#[derive(Debug)]
pub struct Arena {
    pub asteroid_spawn_timer: Timer,
}
pub fn setup_arena(
    commands: Commands,
    mut runstate: ResMut<RunState>,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    runstate.arena = Some(Arena {
        asteroid_spawn_timer: Timer::from_seconds(5.0, false),
    });
    runstate.score = Some(0);
    spawn_player(commands, runstate, asset_server, materials);
}

pub fn spawn_asteroid_event(
    mut commands: Commands,
    mut event_reader: EventReader<AsteroidSpawnEvent>,
    runstate: Res<RunState>,
) {
    for event in event_reader.iter() {
        let (sprite_handle, radius) = match event.size {
            AsteroidSize::Big => (runstate.meteor_big_handle.clone(), 10.1 / 2.0),
            AsteroidSize::Medium => (runstate.meteor_med_handle.clone(), 4.3 / 2.0),
            AsteroidSize::Small => (runstate.meteor_small_handle.clone(), 2.8 / 2.0),
        };
        let mut entity_builder = commands.spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(event.x, event.y, -5.0),
                scale: Vec3::splat(1.0 / 10.0),
                ..Default::default()
            },
            material: sprite_handle.clone(),
            ..Default::default()
        });
        entity_builder
            .insert(Asteroid { size: event.size })
            .insert(Damage { value: 1 })
            .insert(ForState {
                states: vec![AppState::Game],
            });
        let body = RigidBodyBuilder::new_dynamic()
            .translation(event.x, event.y)
            .linvel(event.vx, event.vy)
            .angvel(event.angvel)
            .user_data(entity_builder.id().to_bits() as u128);
        let collider = ColliderBuilder::ball(radius).friction(-0.3);
        entity_builder.insert_bundle((body, collider));
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
            let n_asteroid = asteroids.iter().count();
            arena.asteroid_spawn_timer.reset();
            if n_asteroid < 20 {
                arena
                    .asteroid_spawn_timer
                    .set_duration(Duration::from_secs_f32(
                        (0.8 * arena.asteroid_spawn_timer.duration().as_secs_f32()).max(0.1),
                    ));
                let mut rng = thread_rng();
                // 0: Top , 1:Left
                let side = rng.gen_range(0, 2);
                let (x, y) = match side {
                    0 => (
                        rng.gen_range(-ARENA_WIDTH / 2.0, ARENA_WIDTH / 2.0),
                        ARENA_HEIGHT / 2.0,
                    ),
                    _ => (
                        -ARENA_WIDTH / 2.0,
                        rng.gen_range(-ARENA_HEIGHT / 2.0, ARENA_HEIGHT / 2.0),
                    ),
                };
                let vx = rng.gen_range(-ARENA_WIDTH / 4.0, ARENA_WIDTH / 4.0);
                let vy = rng.gen_range(-ARENA_HEIGHT / 4.0, ARENA_HEIGHT / 4.0);
                let angvel = rng.gen_range(-10.0, 10.0);
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

pub fn position_system(mut bodies: ResMut<RigidBodySet>, query: Query<&RigidBodyHandleComponent>) {
    for body_handle in &mut query.iter() {
        let body = bodies.get_mut(body_handle.handle()).unwrap();
        let mut x = body.position().translation.vector.x;
        let mut y = body.position().translation.vector.y;
        let mut updated = false;
        // Wrap around screen edges
        let half_width = ARENA_WIDTH / 2.0;
        let half_height = ARENA_HEIGHT / 2.0;
        if x < -half_width && body.linvel().x < 0.0 {
            x = half_width;
            updated = true;
        } else if x > half_width && body.linvel().x > 0.0 {
            x = -half_width;
            updated = true;
        }
        if y < -half_height && body.linvel().y < 0.0 {
            y = half_height;
            updated = true;
        } else if y > half_height && body.linvel().y > 0.0 {
            y = -half_height;
            updated = true;
        }
        if updated {
            let mut new_position = *body.position();
            new_position.translation.vector.x = x;
            new_position.translation.vector.y = y;
            body.set_position(new_position, false);
        }
    }
}
