#![allow(clippy::too_many_arguments)]
mod arena;
mod assets;
mod asteroid;
mod background;
mod explosion;
mod hud;
mod laser;
mod menu;
// TODO: Reactivate with Bevy_hanabi uupdate
//mod particle_effects;
mod player_ship;
mod state;

mod prelude {
    pub use crate::arena::*;
    pub use crate::assets::*;
    pub use crate::asteroid::*;
    pub use crate::background::*;
    pub use crate::explosion::*;
    pub use crate::hud::*;
    pub use crate::laser::*;
    pub use crate::menu::*;
    pub use crate::player_ship::*;
    pub use crate::state::*;
    pub use avian2d::prelude::*;
    pub use bevy::prelude::*;
    pub use bevy::reflect::TypePath;
    pub use leafwing_input_manager::prelude::*;
    pub use rand::{thread_rng, Rng};
}

use avian2d::prelude::PhysicsPlugins;
use bevy::{
    remote::{http::RemoteHttpPlugin, RemotePlugin},
    render::{
        batching::gpu_preprocessing::{GpuPreprocessingMode, GpuPreprocessingSupport},
        RenderApp,
    },
    window::WindowResolution,
};

use crate::prelude::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb_u8(0, 0, 0)));
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Kataster".to_string(),
            resolution: WindowResolution::new(ARENA_WIDTH, ARENA_HEIGHT),
            ..default()
        }),
        ..default()
    }));

    // Add some plugins to help debugging only when compiled in debug mode
    #[cfg(debug_assertions)]
    // Enable Avian2d debug renders
    app.add_plugins(PhysicsDebugPlugin::default())
        // Enable connection to external tools, like vscode inspector
        .add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default());

    // Compute shaders are not supported on WASM.
    // TODO: Reactivate with Bevy_hanabi uupdate
    //#[cfg(not(target_arch = "wasm32"))]
    //{
    //    app.add_plugins(particle_effects::ParticleEffectsPlugin);
    //}

    app.add_plugins((
        PhysicsPlugins::default(),
        InputManagerPlugin::<MenuAction>::default(),
    ));

    app.add_plugins((
        StatesPlugin,
        AssetsPlugin,
        ArenaPlugin,
        PlayerShipPlugin,
        LaserPlugin,
        AsteroidPlugin,
        HudPlugin,
        MenuPlugin,
        ExplosionPlugin,
        BackgroundPlugin,
    ));

    app.add_systems(OnEnter(AppState::Setup), setup_camera);

    // On my WSL setup, GpuPreprocessing is detected:
    //  2025-05-12T07:55:11.348347Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
    // But it is not working:
    // Caused by:
    // In Device::create_compute_pipeline, label = 'downsample depth multisample first phase pipeline'
    // Internal error: WGSL `textureLoad` from depth textures is not supported in GLSL
    //
    // Bevy Issue: https://github.com/bevyengine/bevy/issues/18932
    // TODO: Check for update on the issue.
    app.sub_app_mut(RenderApp)
        .insert_resource(GpuPreprocessingSupport {
            max_supported_mode: GpuPreprocessingMode::None,
        });

    app.run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}
