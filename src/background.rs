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
            .add_system(update_background_material);
    }
}

// Spawn a simple stretched quad that will use of backgound shader
fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    // Choose a random f32 for start_time, to have different background
    let mut rng = thread_rng();
    let start_time = rng.gen_range(0.0..100.0f32);
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -10.0),
            scale: Vec3::new(ARENA_WIDTH, ARENA_HEIGHT, 1.0),
            ..Default::default()
        },
        material: materials.add(BackgroundMaterial { time: start_time }),
        ..Default::default()
    });
}

// Currently the time is passed through our BackgroundMaterial
// So we need to update its time attribute apart if we are 'paused' for better UX
fn update_background_material(
    state: Res<State<AppState>>,
    gamestate: Res<State<AppGameState>>,
    time: Res<Time>,
    mut background_materials: ResMut<Assets<BackgroundMaterial>>,
) {
    if state.current() != &AppState::Game || gamestate.current() != &AppGameState::Pause {
        for (_id, mut background_material) in background_materials.iter_mut() {
            background_material.time += time.delta_seconds();
        }
    }
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "d1776d38-712a-11ec-90d6-0242ac120003"]
struct BackgroundMaterial {
    #[uniform(0)]
    time: f32,
}
impl Material2d for BackgroundMaterial {
    fn vertex_shader() -> ShaderRef {
        "background.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "background.wgsl".into()
    }
}
