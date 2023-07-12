#![allow(clippy::too_many_arguments)]
mod arena;
mod assets;
mod asteroid;
mod background;
mod contact;
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
    pub use crate::contact::*;
    pub use crate::explosion::*;
    pub use crate::hud::*;
    pub use crate::laser::*;
    pub use crate::menu::*;
    pub use crate::player_ship::*;
    pub use crate::state::*;
    pub use bevy::prelude::*;
    pub use bevy::reflect::TypePath;
    pub use bevy_rapier2d::prelude::*;
    pub use leafwing_input_manager::prelude::*;
    pub use rand::{thread_rng, Rng};
}

use bevy::window::WindowResolution;

use crate::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_state::<AppState>();

    app.insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)));
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Kataster".to_string(),
            resolution: WindowResolution::new(ARENA_WIDTH, ARENA_HEIGHT),
            ..default()
        }),
        ..default()
    }));

    // Compute shaders are not supported on WASM.
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.add_plugins(particle_effects::ParticleEffectsPlugin);
    }

    // Enable Rapier debug renders when compile in debug mode.
    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_plugins((
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0),
        InputManagerPlugin::<MenuAction>::default(),
    ));

    app.add_plugins((
        AssetsPlugin,
        ArenaPlugin,
        PlayerShipPlugin,
        LaserPlugin,
        AsteroidPlugin,
        HudPlugin,
        MenuPlugin,
        StatesPlugin,
        ContactPlugin,
        ExplosionPlugin,
        BackgroundPlugin,
    ));

    app.add_systems(Startup, setup_camera);
    app.run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
