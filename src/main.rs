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
    narrow_phase::ContactEvent,
    pipeline::{CollisionGroups, CollisionObjectSlabHandle, GeometricQueryType},
    shape::{Ball, ShapeHandle},
    world::CollisionWorld,
};
use rand::prelude::*;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 800;

struct Velocity {
    vx: f32,
    vy: f32,
}
fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "NCollide2D Bevy showcase".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.01, 0.01, 0.03)))
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(spawn_sphere_system.system())
        .add_system(position_system.system())
        .add_system(collision_system.system())
        .run();
}

fn setup(mut commands: Commands) {
    let world = CollisionWorld::<f64, Entity>::new(0.02);
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
    mut world: ResMut<CollisionWorld<f64, Entity>>,
    mut query: Query<(Mut<Translation>, &CollisionObjectSlabHandle, &Velocity)>,
) {
    let elapsed = time.delta_seconds;
    for (mut translation, &handle, velocity) in &mut query.iter() {
        *translation.x_mut() += velocity.vx * elapsed;
        *translation.y_mut() += velocity.vy * elapsed;
        // Wrap around screen edges
        if translation.x() < 0.0 && velocity.vx < 0.0 {
            *translation.x_mut() = WINDOW_WIDTH as f32
        } else if translation.x() > WINDOW_HEIGHT as f32 && velocity.vx > 0.0 {
            *translation.x_mut() = 0.0;
        }
        if translation.y() < 0.0 && velocity.vy < 0.0 {
            *translation.y_mut() = WINDOW_HEIGHT as f32
        } else if translation.y() > WINDOW_HEIGHT as f32 && velocity.vy > 0.0 {
            *translation.y_mut() = 0.0;
        }

        let collision_object = world.get_mut(handle).unwrap();
        collision_object.set_position(Isometry2::new(
            Vector2::new(translation.x() as f64, translation.y() as f64),
            nalgebra::zero(),
        ));
    }
}

fn collision_system(
    mut world: ResMut<CollisionWorld<f64, Entity>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &Handle<ColorMaterial>)>,
) {
    world.update();
    for event in world.contact_events() {
        match event {
            &ContactEvent::Started(collision_handle1, collision_handle2) => {
                let entity1 = *world.collision_object(collision_handle1).unwrap().data();
                let entity2 = *world.collision_object(collision_handle2).unwrap().data();
                for (entity, color) in &mut query.iter() {
                    if entity == entity1 || entity == entity2 {
                        let mut color_mat = materials.get_mut(&*color).unwrap();
                        color_mat.color = Color::rgb(1.0, 0.0, 0.0);
                    }
                }
            }
            _ => (),
        }
    }
}
fn spawn_sphere_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut world: ResMut<CollisionWorld<f64, Entity>>,
    sphere_groups: Res<CollisionGroups>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0, WINDOW_WIDTH as f32);
        let y = rng.gen_range(0.0, WINDOW_HEIGHT as f32);
        let z = rng.gen_range(0.0, 1.0);
        let vx = rng.gen_range(-(WINDOW_WIDTH as f32) / 4.0, (WINDOW_WIDTH as f32) / 4.0);
        let vy = rng.gen_range(-(WINDOW_HEIGHT as f32) / 4.0, (WINDOW_HEIGHT as f32) / 4.0);
        let texture_handle = asset_server
            .load("assets/sprite_sphere_256x256.png")
            .unwrap();
        let shape = ShapeHandle::new(Ball::new(128.0 * 0.1));
        let entity = Entity::new();
        let (collision_object_handle, _) = world.add(
            Isometry2::new(Vector2::new(x as f64, y as f64), nalgebra::zero()),
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
            .with(Velocity { vx, vy })
            .with(collision_object_handle);
    }
}
