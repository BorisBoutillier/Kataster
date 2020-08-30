use bevy::{
    prelude::*,
    render::{camera::OrthographicProjection, pass::ClearColor},
};
use bevy_rapier2d::{
    na::Vector2,
    physics::{EventQueue, Gravity, RapierPhysicsPlugin, RigidBodyHandleComponent},
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodyHandle, RigidBodySet},
        geometry::ColliderBuilder,
        ncollide::narrow_phase::ContactEvent,
        //        math::Point,
    },
};
use rand::{thread_rng, Rng};
use std::collections::HashMap;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 800;
const CAMERA_SCALE: f32 = 0.1;
const ARENA_WIDTH: f32 = WINDOW_WIDTH as f32 * CAMERA_SCALE;
const ARENA_HEIGHT: f32 = WINDOW_HEIGHT as f32 * CAMERA_SCALE;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 2 })
        .add_resource(WindowDescriptor {
            title: "Spaceship 02".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.02, 0.02, 0.04)))
        .add_default_plugins()
        .add_plugin(RapierPhysicsPlugin)
        .add_resource(Gravity(Vector2::zeros()))
        .add_startup_system(setup.system())
        .add_startup_system(spawn_player.system())
        .add_startup_system(spawn_asteroid.system())
        .add_system(position_system.system())
        .add_system(user_input_system.system())
        .add_system(player_dampening_system.system())
        .add_system(body_to_entity_system.system())
        .add_system_to_stage(stage::POST_UPDATE, contact_system.system())
        .add_resource(BodyHandleToEntity(HashMap::new()))
        .run();
}

struct Player(Entity);

struct Ship {
    /// Ship rotation speed in rad/s
    rotation_speed: f32,
    /// Ship thrust N
    thrust: f32,
    /// Ship life points
    life: u32,
}

struct Asteroid {}
struct Damage {
    value: u32,
}

struct BodyHandleToEntity(HashMap<RigidBodyHandle, Entity>);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents {
        orthographic_projection: OrthographicProjection {
            far: 1000.0 / CAMERA_SCALE,
            ..Default::default()
        },
        scale: Scale(CAMERA_SCALE),
        ..Default::default()
    });
}
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("assets/playerShip2_red.png").unwrap();
    let body = RigidBodyBuilder::new_dynamic();
    let collider = ColliderBuilder::ball(1.0);
    // The triangle Collider does not compute mass
    //let collider = ColliderBuilder::triangle(
    //    Point::new(1.0, -0.5),
    //    Point::new(0.0, 0.8),
    //    Point::new(-1.0, -0.5),
    //);
    let player_entity = Entity::new();
    commands
        .spawn_as_entity(
            player_entity,
            SpriteComponents {
                translation: Translation::new(0.0, 0.0, 1.0),
                material: materials.add(texture_handle.into()),
                scale: Scale(1.0 / 37.0),
                ..Default::default()
            },
        )
        .with(Ship {
            rotation_speed: 10.0,
            thrust: 60.0,
            life: 4,
        })
        .with(body)
        .with(collider);
    commands.insert_resource(Player(player_entity));

    // Helper points to visualize some points in space for Collider
    //commands
    //    .spawn(SpriteComponents {
    //        translation: Translation::new(1.2, -1.0, 2.0),
    //        material: materials.add(texture_handle.into()),
    //        scale: Scale(0.001),
    //        ..Default::default()
    //    })
    //    .spawn(SpriteComponents {
    //        translation: Translation::new(0.0, 1.0, 2.0),
    //        material: materials.add(texture_handle.into()),
    //        scale: Scale(0.001),
    //        ..Default::default()
    //    })
    //    .spawn(SpriteComponents {
    //        translation: Translation::new(-1.2, -1.0, 2.0),
    //        material: materials.add(texture_handle.into()),
    //        scale: Scale(0.001),
    //        ..Default::default()
    //    });
}
fn spawn_asteroid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("assets/meteorBrown_big1.png").unwrap();
    // The triangle Collider does not compute mass
    //let collider = ColliderBuilder::triangle(
    //    Point::new(1.0, -0.5),
    //    Point::new(0.0, 0.8),
    //    Point::new(-1.0, -0.5),
    //);
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
    let body = RigidBodyBuilder::new_dynamic()
        .translation(x, y)
        .linvel(vx, vy)
        .angvel(angvel);
    let collider = ColliderBuilder::ball(5.0);
    commands
        .spawn(SpriteComponents {
            translation: Translation::new(x, y, 1.0),
            material: materials.add(texture_handle.into()),
            scale: Scale(1.0 / 10.0),
            ..Default::default()
        })
        .with(Asteroid {})
        .with(Damage { value: 1 })
        .with(body)
        .with(collider);
}

fn position_system(mut bodies: ResMut<RigidBodySet>, mut query: Query<&RigidBodyHandleComponent>) {
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
fn player_dampening_system(
    time: Res<Time>,
    player: Res<Player>,
    mut bodies: ResMut<RigidBodySet>,
    query: Query<&RigidBodyHandleComponent>,
) {
    let elapsed = time.delta_seconds;
    let body_handle = query.get::<RigidBodyHandleComponent>(player.0).unwrap();
    let mut body = bodies.get_mut(body_handle.handle()).unwrap();
    body.angvel = body.angvel * 0.1f32.powf(elapsed);
    body.linvel = body.linvel * 0.8f32.powf(elapsed);
}

fn user_input_system(
    input: Res<Input<KeyCode>>,
    player: Res<Player>,
    mut bodies: ResMut<RigidBodySet>,
    query: Query<(&RigidBodyHandleComponent, &Ship)>,
) {
    let mut rotation = 0;
    let mut thrust = 0;
    if input.pressed(KeyCode::W) {
        thrust += 1
    }
    if input.pressed(KeyCode::S) {
        thrust -= 1
    }
    if input.pressed(KeyCode::A) {
        rotation += 1
    }
    if input.pressed(KeyCode::D) {
        rotation -= 1
    }
    if rotation != 0 || thrust != 0 {
        let body_handle = query.get::<RigidBodyHandleComponent>(player.0).unwrap();
        let mut body = bodies.get_mut(body_handle.handle()).unwrap();
        let ship = query.get::<Ship>(player.0).unwrap();
        if rotation != 0 {
            let rotation = rotation as f32 * ship.rotation_speed;
            body.apply_torque(rotation);
        }
        if thrust != 0 {
            let force = body.position.rotation.transform_vector(&Vector2::y())
                * thrust as f32
                * ship.thrust;
            body.apply_force(force);
        }
    }
}

fn contact_system(
    events: Res<EventQueue>,
    h_to_e: Res<BodyHandleToEntity>,
    damages: Query<&Damage>,
    ships: Query<Mut<Ship>>,
) {
    while let Ok(contact_event) = events.contact_events.pop() {
        match contact_event {
            ContactEvent::Started(h1, h2) => {
                let e1 = h_to_e.0.get(&h1).unwrap();
                let e2 = h_to_e.0.get(&h2).unwrap();
                if let Ok(mut ship) = ships.get_mut::<Ship>(*e1) {
                    if let Ok(damage) = damages.get::<Damage>(*e2) {
                        ship.life -= damage.value;
                        if ship.life <= 0 {
                            println!("Player DEAD")
                        } else {
                            println!("Player contact Life: {}", ship.life)
                        }
                    }
                }
                if let Ok(mut ship) = ships.get_mut::<Ship>(*e2) {
                    if let Ok(damage) = damages.get::<Damage>(*e1) {
                        ship.life -= damage.value;
                        if ship.life <= 0 {
                            println!("Player DEAD")
                        } else {
                            println!("Player contact remains {}", ship.life)
                        }
                    }
                }
            }
            _ => (),
        };
    }
}

fn body_to_entity_system(
    mut h_to_e: ResMut<BodyHandleToEntity>,
    mut added: Query<(Entity, Added<RigidBodyHandleComponent>)>,
) {
    for (entity, body_handle) in &mut added.iter() {
        h_to_e.0.insert(body_handle.handle(), entity);
    }
}
