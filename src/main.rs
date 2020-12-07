use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;
use bevy_rapier2d::physics::RapierPhysicsPlugin;

mod arena;
mod components;
mod contact;
mod explosion;
mod laser;
mod player;
mod state;
mod ui;

use arena::*;
use bevy_rapier2d::na::Vector2;
use components::*;
use contact::*;
use explosion::*;
use laser::*;
use player::*;
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
        .add_plugins(DefaultPlugins)
        .add_resource(RapierConfiguration {
            gravity: Vector2::zeros(),
            ..Default::default()
        })
        .add_system(position_system)
        .add_system(user_input_system)
        .add_system(player_dampening_system)
        .add_system(ship_cannon_system)
        .add_system(despawn_laser_system)
        .add_system(handle_explosion)
        .add_system(setup_arena)
        .add_system(arena_spawn)
        .add_system(start_menu)
        .add_system(game_ui_spawn)
        .add_system(score_ui_system)
        .add_system(life_ui_system)
        .add_system(gameover_menu)
        .add_system(pause_menu)
        .add_system(draw_blink_system)
        .add_system(state_exit_despawn)
        .add_system(contact_system)
        .add_system(spawn_asteroid_system)
        .add_system(spawn_explosion)
        .add_system(runstate_fsm)
        .add_startup_system(setup)
        .run();
}

/// UiCamera and Camera2d are spawn once and for all.
/// Despawning them does not seem to be the way to go in bevy.
pub fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dBundle {
            transform: Transform {
                scale: Vec3::splat(CAMERA_SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .spawn(CameraUiBundle::default());
    let texture_handle = asset_server.load("pexels-francesco-ungaro-998641.png");
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -10.0),
            scale: Vec3::splat(CAMERA_SCALE),
            ..Default::default()
        },
        material: materials.add(texture_handle.into()),
        ..Default::default()
    });
    commands.insert_resource(RunState::new(
        GameState::StartMenu,
        &asset_server,
        materials,
    ));
}
