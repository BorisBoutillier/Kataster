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

#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
struct DespawnLaserLabel;

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
        .add_event::<LaserDespawnEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(BackgroundPlugin {})
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
                .with_system(player_dampening_system)
                .with_system(ship_cannon_system)
                .with_system(laser_timeout_system.label(DespawnLaserLabel))
                .with_system(contact_system.label(DespawnLaserLabel))
                .with_system(arena_asteroids)
                .with_system(spawn_asteroid_event)
                .with_system(score_ui_system)
                .with_system(life_ui_system)
                .with_system(despawn_laser_system.after(DespawnLaserLabel)),
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
        .add_system(user_input_system)
        .add_system(handle_explosion)
        .add_system(draw_blink_system)
        .add_system(spawn_explosion_event)
        .add_startup_system(setup)
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
