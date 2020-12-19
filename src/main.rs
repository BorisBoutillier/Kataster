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
            width: WINDOW_WIDTH as f32,
            height: WINDOW_HEIGHT as f32,
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
        .add_resource(State::new(AppState::StartMenu))
        .add_stage_after(
            stage::UPDATE,
            APPSTATE_STAGE,
            StateStage::<AppState>::default(),
        )
        .stage(APPSTATE_STAGE, |stage: &mut StateStage<AppState>| {
            stage
                .on_state_enter(AppState::StartMenu, start_menu.system())
                .on_state_exit(AppState::StartMenu, appstate_exit_despawn.system())
                .on_state_enter(AppState::Game, setup_arena.system())
                .on_state_enter(AppState::Game, game_ui_spawn.system())
                .update_stage(AppState::Game, |stage: &mut SystemStage| {
                    stage
                        .add_system(position_system.system())
                        .add_system(position_system.system())
                        .add_system(player_dampening_system.system())
                        .add_system(ship_cannon_system.system())
                        .add_system(despawn_laser_system.system())
                        .add_system(contact_system.system())
                        .add_system(arena_asteroids.system())
                        .add_system(spawn_asteroid_event.system())
                        .add_system(score_ui_system.system())
                        .add_system(life_ui_system.system())
                })
                .on_state_exit(AppState::Game, appstate_exit_despawn.system())
        })
        .add_resource(State::new(AppGameState::Invalid))
        .add_stage_after(
            APPSTATE_STAGE,
            APPGAMESTATE_STAGE,
            StateStage::<AppGameState>::default(),
        )
        .stage(
            APPGAMESTATE_STAGE,
            |stage: &mut StateStage<AppGameState>| {
                stage
                    .on_state_enter(AppGameState::Pause, pause_menu.system())
                    .on_state_exit(AppGameState::Pause, appgamestate_exit_despawn.system())
                    .on_state_enter(AppGameState::GameOver, gameover_menu.system())
                    .on_state_exit(AppGameState::GameOver, appgamestate_exit_despawn.system())
            },
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
