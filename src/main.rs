use bevy::{
    prelude::*,
    render::{
        camera::{OrthographicProjection, WindowOrigin},
        pass::ClearColor,
    },
};
use ncollide2d::{
    nalgebra,
    nalgebra::{Isometry2, Vector2},
    pipeline::{CollisionGroups, CollisionObjectSlabHandle, GeometricQueryType},
    shape::{Ball, ShapeHandle},
    world::CollisionWorld,
};
use rand::prelude::*;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 800;

#[derive(Default)]
struct MouseState {
    cursor_moved_events: EventReader<CursorMoved>,
    position: Vec2,
}
struct Velocity(Vector2<f32>);
fn main() {
    App::build()
        .init_resource::<MouseState>()
        .add_resource(WindowDescriptor {
            title: "NCollide2D Bevy showcase".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.01, 0.01, 0.03)))
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(mouse_position_system.system())
        .add_system(spawn_sphere_system.system())
        .add_system(position_system.system())
        .add_system(collision_system.system())
        .run();
}

fn setup(mut commands: Commands) {
    let world = CollisionWorld::<f32, Entity>::new(0.02);
    let mut sphere_groups = CollisionGroups::new();
    sphere_groups.set_membership(&[1]);
    commands.spawn(Camera2dComponents {
        orthographic_projection: OrthographicProjection {
            window_origin: WindowOrigin::BottomLeft,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.insert_resource(sphere_groups);
    commands.insert_resource(world);
}

fn position_system(
    time: Res<Time>,
    mut world: ResMut<CollisionWorld<f32, Entity>>,
    mut query: Query<(Mut<Translation>, &CollisionObjectSlabHandle, &Velocity)>,
) {
    let elapsed = time.delta_seconds;
    for (mut translation, &handle, velocity) in &mut query.iter() {
        *translation.x_mut() += velocity.0.x * elapsed;
        *translation.y_mut() += velocity.0.y * elapsed;
        // Wrap around screen edges
        if translation.x() < 0.0 && velocity.0.x < 0.0 {
            *translation.x_mut() = WINDOW_WIDTH as f32
        } else if translation.x() > WINDOW_HEIGHT as f32 && velocity.0.x > 0.0 {
            *translation.x_mut() = 0.0;
        }
        if translation.y() < 0.0 && velocity.0.y < 0.0 {
            *translation.y_mut() = WINDOW_HEIGHT as f32
        } else if translation.y() > WINDOW_HEIGHT as f32 && velocity.0.y > 0.0 {
            *translation.y_mut() = 0.0;
        }

        let collision_object = world.get_mut(handle).unwrap();
        collision_object.set_position(Isometry2::new(
            Vector2::new(translation.x() as f32, translation.y() as f32),
            nalgebra::zero(),
        ));
    }
}

fn collision_system(
    mut world: ResMut<CollisionWorld<f32, Entity>>,
    mut velocities: Query<(Entity, Mut<Velocity>)>,
) {
    world.update();
    for (h1, h2, _, manifold) in world.contact_pairs(true) {
        if let Some(tracked_contact) = manifold.deepest_contact() {
            let contact_normal = tracked_contact.contact.normal.into_inner();
            let entity1 = *world.collision_object(h1).unwrap().data();
            let entity2 = *world.collision_object(h2).unwrap().data();
            for (entity, mut velocity) in &mut velocities.iter() {
                if entity == entity1 || entity == entity2 {
                    *velocity = Velocity(reflect(velocity.0, contact_normal));
                }
            }
        }
    }
}
fn mouse_position_system(
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut event_readers: ResMut<MouseState>,
) {
    for event in event_readers.cursor_moved_events.iter(&cursor_moved_events) {
        event_readers.position = event.position;
    }
}
fn spawn_sphere_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut world: ResMut<CollisionWorld<f32, Entity>>,
    sphere_groups: Res<CollisionGroups>,
    mouse_state: Res<MouseState>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut rng = thread_rng();
        let x = mouse_state.position.x();
        let y = mouse_state.position.y();
        let z = rng.gen_range(0.0, 1.0);
        let vx = rng.gen_range(-(WINDOW_WIDTH as f32) / 4.0, (WINDOW_WIDTH as f32) / 4.0);
        let vy = rng.gen_range(-(WINDOW_HEIGHT as f32) / 4.0, (WINDOW_HEIGHT as f32) / 4.0);
        let texture_handle = asset_server
            .load("assets/sprite_sphere_256x256.png")
            .unwrap();
        let shape = ShapeHandle::new(Ball::new(128.0 * 0.1));
        let entity = Entity::new();
        let (collision_object_handle, _) = world.add(
            Isometry2::new(Vector2::new(x as f32, y as f32), nalgebra::zero()),
            shape,
            *sphere_groups,
            GeometricQueryType::Contacts(0.0, 0.0),
            entity,
        );
        commands
            .spawn_as_entity(
                entity,
                SpriteComponents {
                    translation: Translation::new(x, y, z),
                    material: materials.add(texture_handle.into()),
                    scale: Scale(0.1),
                    ..Default::default()
                },
            )
            .with(Velocity(Vector2::new(vx, vy)))
            .with(collision_object_handle);
    }
}

fn reflect(d: Vector2<f32>, n: Vector2<f32>) -> Vector2<f32> {
    d - 2.0 * (d.dot(&n)) * n
}
