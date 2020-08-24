use bevy::{prelude::*, render::pass::ClearColor};
use rand::prelude::*;

struct Velocity {
    vx: f32,
    vy: f32,
}
fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.2, 0.2, 0.8)))
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(spawn_sphere_system.system())
        .add_system(position_system.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}

fn position_system(time: Res<Time>, mut query: Query<(Mut<Translation>, &Velocity)>) {
    let elapsed = time.delta_seconds;
    for (mut translation, velocity) in &mut query.iter() {
        *translation.x_mut() += velocity.vx * elapsed;
        *translation.y_mut() += velocity.vy * elapsed;
    }
}
fn spawn_sphere_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut rng = thread_rng();
        let x = rng.gen_range(-200.0, 200.0);
        let y = rng.gen_range(-200.0, 200.0);
        let vx = rng.gen_range(-100.0, 100.0);
        let vy = rng.gen_range(-100.0, 100.0);
        let texture_handle = asset_server
            .load("assets/sprite_sphere_256x256.png")
            .unwrap();
        commands
            .spawn(SpriteComponents {
                translation: Translation::new(x, y, 1.0),
                material: materials.add(texture_handle.into()),
                scale: Scale(0.1),
                ..Default::default()
            })
            .with(Velocity { vx, vy });
    }
}
