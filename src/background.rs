use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2dPlugin;
use bevy::{
    reflect::TypeUuid,
    sprite::{Material2d, MaterialMesh2dBundle},
};

use crate::prelude::*;

// Plugin that will insert a background at Z = -10.0, use the custom 'Star Nest' shader
pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<BackgroundMaterial>::default())
            .add_startup_system(spawn_background)
            .add_system(update_background_time);
    }
}

// Spawn a simple stretched quad that will use of backgound shader
fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(ARENA_WIDTH, ARENA_HEIGHT, 1.0),
            ..Default::default()
        },
        material: materials.add(BackgroundMaterial { time: 0.0 }),
        ..Default::default()
    });
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "d1776d38-712a-11ec-90d6-0242ac120003"]
struct BackgroundMaterial {
    #[uniform(0)]
    time: f32,
}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "background.wgsl".into()
    }
}

fn update_background_time(
    time: Res<Time>,
    state: Res<State<AppState>>,
    mut backgrounds: ResMut<Assets<BackgroundMaterial>>,
) {
    if state.0 != AppState::GamePaused {
        for (_, background) in backgrounds.iter_mut() {
            background.time += time.delta_seconds();
        }
    }
}
