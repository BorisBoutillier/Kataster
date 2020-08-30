use bevy::{prelude::*, render::camera::OrthographicProjection};
use bevy_rapier2d::{
    physics::RigidBodyHandleComponent,
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
        //        math::Point,
    },
};
use rand::{thread_rng, Rng};

use super::components::*;

pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 800;
const CAMERA_SCALE: f32 = 0.1;
const ARENA_WIDTH: f32 = WINDOW_WIDTH as f32 * CAMERA_SCALE;
const ARENA_HEIGHT: f32 = WINDOW_HEIGHT as f32 * CAMERA_SCALE;

pub struct Arena {
    pub asteroid_spawn_timer: Timer,
}
pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents {
        orthographic_projection: OrthographicProjection {
            far: 1000.0 / CAMERA_SCALE,
            ..Default::default()
        },
        scale: Scale(CAMERA_SCALE),
        ..Default::default()
    });
    commands.insert_resource(Arena {
        asteroid_spawn_timer: Timer::from_seconds(5.0, false),
    });
}
pub fn spawn_asteroid(
    mut commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    size: AsteroidSize,
    (x, y): (f32, f32),
    (vx, vy): (f32, f32),
    angvel: f32,
) {
    let texture_handle = asset_server.load("assets/meteorBrown_big1.png").unwrap();
    let body = RigidBodyBuilder::new_dynamic()
        .translation(x, y)
        .linvel(vx, vy)
        .angvel(angvel);
    let collider = ColliderBuilder::ball(5.0).friction(-0.3);
    commands
        .spawn(SpriteComponents {
            translation: Translation::new(x, y, 1.0),
            material: materials.add(texture_handle.into()),
            scale: Scale(1.0 / 10.0),
            ..Default::default()
        })
        .with(Asteroid { size })
        .with(Damage { value: 1 })
        .with(body)
        .with(collider);
}

pub fn spawn_random_asteroid(
    mut commands: Commands,
    time: Res<Time>,
    mut arena: ResMut<Arena>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asteroids: Query<&Asteroid>,
) {
    arena.asteroid_spawn_timer.tick(time.delta_seconds);
    if arena.asteroid_spawn_timer.finished {
        let n_asteroid = asteroids.iter().iter().count();
        arena.asteroid_spawn_timer.reset();
        if n_asteroid < 20 {
            arena.asteroid_spawn_timer.duration =
                (0.8 * arena.asteroid_spawn_timer.duration).max(0.1);
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
            spawn_asteroid(
                &mut commands,
                asset_server,
                materials,
                AsteroidSize::Big,
                (x, y),
                (vx, vy),
                angvel,
            );
        }
    }
}

pub fn position_system(
    mut bodies: ResMut<RigidBodySet>,
    mut query: Query<&RigidBodyHandleComponent>,
) {
    for body_handle in &mut query.iter() {
        let mut body = bodies.get_mut(body_handle.handle()).unwrap();
        let mut x = body.position.translation.vector.x;
        let mut y = body.position.translation.vector.y;
        let mut updated = false;
        // Wrap around screen edges
        let half_width = ARENA_WIDTH / 2.0;
        let half_height = ARENA_HEIGHT / 2.0;
        if x < -half_width && body.linvel.x < 0.0 {
            x = half_width;
            updated = true;
        } else if x > half_width && body.linvel.x > 0.0 {
            x = -half_width;
            updated = true;
        }
        if y < -half_height && body.linvel.y < 0.0 {
            y = half_height;
            updated = true;
        } else if y > half_height && body.linvel.y > 0.0 {
            y = -half_height;
            updated = true;
        }
        if updated {
            let mut new_position = body.position.clone();
            new_position.translation.vector.x = x;
            new_position.translation.vector.y = y;
            body.set_position(new_position);
        }
    }
}
