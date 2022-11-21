#![allow(clippy::too_many_arguments)]
mod arena;
mod asteroid;
mod background;
mod components;
mod contact;
mod explosion;
mod laser;
mod particle_effects;
mod player;
mod state;
mod ui;

mod prelude {
    pub use crate::arena::*;
    pub use crate::asteroid::*;
    pub use crate::background::*;
    pub use crate::components::*;
    pub use crate::contact::*;
    pub use crate::explosion::*;
    pub use crate::laser::*;
    pub use crate::player::*;
    pub use crate::state::*;
    pub use crate::ui::*;
    pub use bevy::prelude::*;
    pub use bevy_rapier2d::prelude::*;
    pub use leafwing_input_manager::prelude::*;
    pub use rand::{thread_rng, Rng};
}

use crate::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Kataster".to_string(),
            width: WINDOW_WIDTH as f32,
            height: WINDOW_HEIGHT as f32,
            ..Default::default()
        },
        ..Default::default()
    }));

    // These two plugins are currently not supported on the web
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.add_plugin(BackgroundPlugin {});
        // FIXME: Hanabi currently does not support a game state without any particle effect
        // to be reactivated when issue is checked with hanabi
        //app.add_plugin(particle_effects::ParticleEffectsPlugin);
    }

    // Enable Rapier debug renders when compile in debug mode.
    #[cfg(debug_assertions)]
    app.add_plugin(RapierDebugRenderPlugin::default());

    app.add_plugin(PlayerShipPlugin);
    app.add_plugin(LaserPlugin);
    app.add_plugin(AsteroidPlugin);
    app.insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)))
        .add_event::<ExplosionSpawnEvent>()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0));
    app.add_plugin(InputManagerPlugin::<MenuAction>::default())
        .add_state(AppState::StartMenu)
        .add_system_set(
            SystemSet::on_enter(AppState::StartMenu)
                .with_system(start_menu)
                .with_system(appstate_enter_despawn),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Game)
                .with_system(setup_arena)
                .with_system(game_ui_spawn)
                .with_system(appstate_enter_despawn),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(position_system)
                .with_system(contact_system.label(CanDespawnLaserLabel))
                .with_system(score_ui_system)
                .with_system(life_ui_system),
        )
        .add_state(AppGameState::Invalid)
        .add_system_set(
            SystemSet::on_enter(AppGameState::Pause)
                .with_system(pause_menu)
                .with_system(appgamestate_enter_despawn),
        )
        .add_system_set(
            SystemSet::on_enter(AppGameState::GameOver)
                .with_system(gameover_menu)
                .with_system(appgamestate_enter_despawn),
        )
        .add_system_set(
            SystemSet::on_enter(AppGameState::Invalid).with_system(appgamestate_enter_despawn),
        )
        .add_system_set(
            SystemSet::on_enter(AppGameState::Game).with_system(appgamestate_enter_despawn),
        )
        .add_system(ui_input_system)
        .add_system(handle_explosion)
        .add_system(draw_blink_system)
        .add_system(spawn_explosion_event)
        .add_startup_system(setup)
        .run();
}

/// Camera for both 2D and UI is spawn once and for all.
/// MenuAction InputMap and ActionState are added as global resource to handle Menu interaction
/// Rapier configuration is updated to remove gravity
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    // Camera
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::splat(CAMERA_SCALE),
            ..Default::default()
        },
        ..Default::default()
    });

    // RunState
    commands.insert_resource(RunState::new(&asset_server));

    // Insert MenuAction resources
    commands.insert_resource(InputMap::<MenuAction>::new([
        (KeyCode::Return, MenuAction::Accept),
        (KeyCode::Escape, MenuAction::PauseUnpause),
        (KeyCode::Back, MenuAction::ExitToMenu),
        (KeyCode::Escape, MenuAction::Quit),
    ]));
    commands.insert_resource(ActionState::<MenuAction>::default());

    // Rapier configuration
    rapier_configuration.gravity = Vec2::ZERO;
}
