use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;
use bevy_rapier2d::physics::RapierPhysicsPlugin;

mod arena;
mod components;
mod contact;
mod explosion;
mod laser;
mod player;
mod rapier2d;
mod state;
mod ui;

use arena::*;
use bevy_rapier2d::na::Vector2;
use components::*;
use contact::*;
use explosion::*;
use laser::*;
use player::*;
use rapier2d::*;
use state::*;
use ui::*;
const START_LIFE: u32 = 3;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Kataster".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb_u8(5, 5, 10)))
        .add_event::<AsteroidSpawnEvent>()
        .add_event::<ExplosionSpawnEvent>()
        .add_plugin(RapierPhysicsPlugin)
        .add_default_plugins()
        .add_resource(RapierConfiguration {
            gravity: Vector2::zeros(),
            ..Default::default()
        })
        // Stage added after add_default_plugins, else something messes up CLEANUP
        .add_stage_after(stage::POST_UPDATE, "HANDLE_CONTACT")
        .add_stage_after("HANDLE_CONTACT", "HANDLE_EXPLOSION")
        .add_stage_after("HANDLE_EXPLOSION", "HANDLE_RUNSTATE")
        .add_stage_after("HANDLE_RUNSTATE", "CLEANUP") // CLEANUP stage required by RapierUtilsPlugin
        .add_plugin(RapierUtilsPlugin)
        .add_system(position_system.system())
        .add_system(user_input_system.system())
        .add_system(player_dampening_system.system())
        .add_system(ship_cannon_system.system())
        .add_system(despawn_laser_system.system())
        .add_system(handle_explosion.system())
        .add_system(setup_arena.system())
        .add_system(arena_spawn.system())
        .add_system(start_menu.system())
        .add_system(game_ui_spawn.system())
        .add_system(score_ui_system.system())
        .add_system(life_ui_system.system())
        .add_system(gameover_menu.system())
        .add_system(pause_menu.system())
        .add_system(draw_blink_system.system())
        .add_system(state_exit_despawn.system())
        .add_startup_system(setup.system())
        .add_system_to_stage(stage::POST_UPDATE, contact_system.system())
        .add_system_to_stage("HANDLE_CONTACT", spawn_asteroid_system.system())
        .add_system_to_stage("HANDLE_EXPLOSION", spawn_explosion.system())
        .add_system_to_stage("HANDLE_RUNSTATE", runstate_fsm.system())
        .add_resource(RunState::new(GameState::StartMenu))
        .run();
}

/// UiCamera and Camera2d are spawn once and for all.
/// Despawning them does not seem to be the way to go in bevy.
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dComponents {
            transform: Transform::from_scale(CAMERA_SCALE),
            ..Default::default()
        })
        .spawn(UiCameraComponents::default());
    let texture_handle = asset_server
        .load("assets/pexels-francesco-ungaro-998641.png")
        .unwrap();
    commands.spawn(SpriteComponents {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)).with_scale(CAMERA_SCALE),
        material: materials.add(texture_handle.into()),
        ..Default::default()
    });
}
