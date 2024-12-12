use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;
use bevy::sprite::Material2dPlugin;

use crate::prelude::*;

// Plugin that will insert a background at Z = -10.0, use the custom 'Star Nest' shader
pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<BackgroundMaterial>::default())
            .add_systems(OnEnter(AppState::Setup), spawn_background)
            .add_systems(Update, update_background_time);
    }
}

// Spawn a simple stretched quad that will use of background shader
fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(ARENA_WIDTH, ARENA_HEIGHT, 1.0),
            ..default()
        },
        MeshMaterial2d(materials.add(BackgroundMaterial { time: 0.0 })),
    ));
}

#[derive(Asset, AsBindGroup, Debug, Clone, TypePath)]
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
    state: Option<Res<State<GameState>>>,
    mut backgrounds: ResMut<Assets<BackgroundMaterial>>,
) {
    if state.is_none() || state.unwrap().get() != &GameState::Paused {
        for (_, background) in backgrounds.iter_mut() {
            background.time += time.delta_secs();
        }
    }
}
