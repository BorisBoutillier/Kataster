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
        .add_state(AppState::StartMenu)
        .add_state(AppGameState::Game)
        .state_enter(AppState::StartMenu, start_menu)
        .state_enter(AppGameState::GameOver, gameover_menu)
        .state_enter(AppGameState::Pause, pause_menu)
        .state_enter(
            AppState::Game,
            SystemStage::parallel()
                .with_system(setup_arena)
                .with_system(game_ui_spawn),
        )
        .state_update(
            AppState::Game,
            SystemStage::parallel()
                .with_system(position_system)
                .with_system(player_dampening_system)
                .with_system(ship_cannon_system)
                .with_system(despawn_laser_system)
                .with_system(contact_system)
                .with_system(arena_asteroids)
                .with_system(spawn_asteroid_event)
                .with_system(score_ui_system)
                .with_system(life_ui_system),
        )
        .state_exit(AppState::Game, appstate_exit_despawn)
        .state_exit(AppGameState::GameOver, appgamestate_exit_despawn)
        .state_exit(AppGameState::Pause, appgamestate_exit_despawn)
        .state_exit(AppState::StartMenu, appstate_exit_despawn)
        .add_system(user_input_system)
        .add_system(handle_explosion)
        .add_system(draw_blink_system)
        .add_system(spawn_explosion_event)
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
    commands.insert_resource(RunState::new(&asset_server, materials));
}
