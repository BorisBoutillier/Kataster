use super::components::*;
use super::player::*;
use super::state::*;
use bevy::prelude::*;
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
    commands: &mut Commands,
    mut runstate: ResMut<RunState>,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    if runstate
        .gamestate
        .entering_not_from(GameState::Game, GameState::Pause)
    {
        runstate.arena = Some(Arena {
            asteroid_spawn_timer: Timer::from_seconds(5.0, false),
        });
        runstate.score = Some(0);
        spawn_player(commands, runstate, asset_server, materials);
    }
}

#[derive(Default)]
pub struct SpawnAsteroidState {
    event_reader: EventReader<AsteroidSpawnEvent>,
}

pub fn spawn_asteroid_system(
    commands: &mut Commands,
    mut local_state: Local<SpawnAsteroidState>,
    runstate: Res<RunState>,
    events: Res<Events<AsteroidSpawnEvent>>,
) {
    for event in local_state.event_reader.iter(&events) {
        let (sprite_handle, radius) = match event.size {
            AsteroidSize::Big => (runstate.meteor_big_handle.clone(), 10.1 / 2.0),
            AsteroidSize::Medium => (runstate.meteor_med_handle.clone(), 4.3 / 2.0),
            AsteroidSize::Small => (runstate.meteor_small_handle.clone(), 2.8 / 2.0),
        };
        let entity = commands
            .spawn(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(event.x, event.y, -5.0),
                    scale: Vec3::splat(1.0 / 10.0),
                    ..Default::default()
                },
                material: sprite_handle.clone(),
                ..Default::default()
            })
            .with(Asteroid { size: event.size })
            .with(Damage { value: 1 })
            .with(ForStates {
                states: vec![GameState::Game, GameState::Pause, GameState::GameOver],
            })
            .current_entity()
            .unwrap();
        let body = RigidBodyBuilder::new_dynamic()
            .translation(event.x, event.y)
            .linvel(event.vx, event.vy)
            .angvel(event.angvel)
            .user_data(entity.to_bits() as u128);
        let collider = ColliderBuilder::ball(radius).friction(-0.3);
        commands.insert(entity, (body, collider));
    }
}

pub fn arena_spawn(
    time: Res<Time>,
    mut runstate: ResMut<RunState>,
    mut asteroid_spawn_events: ResMut<Events<AsteroidSpawnEvent>>,
    asteroids: Query<&Asteroid>,
) {
    if runstate.gamestate.is(GameState::Game) {
        let arena = runstate.arena.as_mut().unwrap();
        arena.asteroid_spawn_timer.tick(time.delta_seconds());
        if arena.asteroid_spawn_timer.finished() {
            let n_asteroid = asteroids.iter().count();
            arena.asteroid_spawn_timer.reset();
            if n_asteroid < 20 {
                arena
                    .asteroid_spawn_timer
                    .set_duration((0.8 * arena.asteroid_spawn_timer.duration()).max(0.1));
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

pub fn position_system(
    runstate: Res<RunState>,
    mut bodies: ResMut<RigidBodySet>,
    query: Query<&RigidBodyHandleComponent>,
) {
    if runstate.gamestate.is(GameState::Game) {
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
                let mut new_position = body.position().clone();
                new_position.translation.vector.x = x;
                new_position.translation.vector.y = y;
                body.set_position(new_position, false);
            }
        }
    }
}
