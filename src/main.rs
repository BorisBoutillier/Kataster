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
//mod particle_effects;
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
            ..Default::default()
        }),
        ..Default::default()
    }));

    // Compute shaders are not supported on WASM.
    #[cfg(not(target_arch = "wasm32"))]
    {
        //app.add_plugin(particle_effects::ParticleEffectsPlugin);
    }

    // Enable Rapier debug renders when compile in debug mode.
    #[cfg(debug_assertions)]
    app.add_plugin(RapierDebugRenderPlugin::default());

    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0));
    app.add_plugin(InputManagerPlugin::<MenuAction>::default());

    app.add_plugin(AssetsPlugin)
        .add_plugin(ArenaPlugin)
        .add_plugin(PlayerShipPlugin)
        .add_plugin(LaserPlugin)
        .add_plugin(AsteroidPlugin)
        .add_plugin(HudPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(StatesPlugin)
        .add_plugin(ContactPlugin)
        .add_plugin(ExplosionPlugin)
        .add_plugin(BackgroundPlugin);

    app.add_startup_system(setup_camera);
    app.run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
