#![allow(clippy::too_many_arguments)]
mod arena;
mod assets;
mod asteroid;
mod background;
mod explosion;
mod hud;
mod laser;
mod menu;
mod particle_effects;
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
    pub use rand::{Rng, thread_rng};
}

use avian2d::prelude::PhysicsPlugins;
use bevy::remote::{RemotePlugin, http::RemoteHttpPlugin};

use crate::prelude::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb_u8(0, 0, 0)));
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Kataster".to_string(),
            resolution: (ARENA_WIDTH as u32, ARENA_HEIGHT as u32).into(),
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
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.add_plugins(particle_effects::ParticleEffectsPlugin);
    }

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

    app.run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}
