#![allow(clippy::too_many_arguments)]
mod arena;
mod background;
mod components;
mod contact;
mod explosion;
mod laser;
mod player;
mod state;
mod ui;

mod prelude {
    pub use crate::arena::*;
    pub use crate::background::*;
    pub use crate::components::*;
    pub use crate::contact::*;
    pub use crate::explosion::*;
    pub use crate::laser::*;
    pub use crate::player::*;
    pub use crate::state::*;
    pub use crate::ui::*;
    pub use bevy::prelude::*;
    pub use heron::prelude::*;
    pub use rand::{thread_rng, Rng};
}

use crate::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Kataster".to_string(),
            width: WINDOW_WIDTH as f32,
            height: WINDOW_HEIGHT as f32,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)))
        .add_event::<AsteroidSpawnEvent>()
        .add_event::<ExplosionSpawnEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(BackgroundPlugin {})
        .add_state(AppState::StartMenu)
        .add_system_set(
            SystemSet::on_enter(AppState::StartMenu)
                .with_system(start_menu.system())
                .with_system(appstate_enter_despawn.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Game)
                .with_system(setup_arena.system())
                .with_system(game_ui_spawn.system())
                .with_system(appstate_enter_despawn.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(position_system.system())
                .with_system(player_dampening_system.system())
                .with_system(ship_cannon_system.system())
                .with_system(despawn_laser_system.system())
                .with_system(contact_system.system())
                .with_system(arena_asteroids.system())
                .with_system(spawn_asteroid_event.system())
                .with_system(score_ui_system.system())
                .with_system(life_ui_system.system()),
        )
        .add_state(AppGameState::Invalid)
        .add_system_set(
            SystemSet::on_enter(AppGameState::Pause)
                .with_system(pause_menu.system())
                .with_system(appgamestate_enter_despawn.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppGameState::GameOver)
                .with_system(gameover_menu.system())
                .with_system(appgamestate_enter_despawn.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppGameState::Invalid)
                .with_system(appgamestate_enter_despawn.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppGameState::Game)
                .with_system(appgamestate_enter_despawn.system()),
        )
        .add_system(user_input_system.system())
        .add_system(handle_explosion.system())
        .add_system(draw_blink_system.system())
        .add_system(spawn_explosion_event.system())
        .add_startup_system(setup.system())
        .run();
}

/// UiCamera and Camera2d are spawn once and for all.
/// Despawning them does not seem to be the way to go in bevy.
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform {
        scale: Vec3::splat(CAMERA_SCALE),
        ..Default::default()
    };
    commands.spawn_bundle(camera);
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(RunState::new(&asset_server));
}
