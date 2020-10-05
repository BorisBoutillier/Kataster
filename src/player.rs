use bevy::app::AppExit;

use super::components::*;
use super::laser::*;
use super::state::*;
use super::START_LIFE;
use bevy::prelude::*;
use bevy_rapier2d::{
    na::Vector2,
    physics::{RapierConfiguration, RigidBodyHandleComponent},
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
        //        math::Point,
    },
};

pub fn spawn_player(
    mut commands: Commands,
    mut runstate: ResMut<RunState>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("assets/playerShip2_red.png").unwrap();
    let body = RigidBodyBuilder::new_dynamic();
    let collider = ColliderBuilder::ball(1.0);
    // The triangle Collider does not compute mass
    //let collider = ColliderBuilder::triangle(
    //    Point::new(1.0, -0.5),
    //    Point::new(0.0, 0.8),
    //    Point::new(-1.0, -0.5),
    //);
    commands
        .spawn(SpriteComponents {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -5.0))
                .with_scale(1.0 / 37.0),
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with(Ship {
            rotation_speed: 0.3,
            thrust: 60.0,
            life: START_LIFE,
            cannon_timer: Timer::from_seconds(0.2, false),
        })
        .with(body)
        .with(collider)
        .with(ForStates {
            states: vec![GameState::Game, GameState::Pause, GameState::GameOver],
        });
    let player_entity = commands.current_entity().unwrap();
    runstate.player = Some(player_entity);

    // Helper points to visualize some points in space for Collider
    //commands
    //    .spawn(SpriteComponents {
    //        translation: Translation::new(1.2, -1.0, 2.0),
    //        material: materials.add(texture_handle.into()),
    //        scale: Scale(0.001),
    //        ..Default::default()
    //    })
    //    .spawn(SpriteComponents {
    //        translation: Translation::new(0.0, 1.0, 2.0),
    //        material: materials.add(texture_handle.into()),
    //        scale: Scale(0.001),
    //        ..Default::default()
    //    })
    //    .spawn(SpriteComponents {
    //        translation: Translation::new(-1.2, -1.0, 2.0),
    //        material: materials.add(texture_handle.into()),
    //        scale: Scale(0.001),
    //        ..Default::default()
    //    });
}

pub fn player_dampening_system(
    time: Res<Time>,
    runstate: Res<RunState>,
    mut bodies: ResMut<RigidBodySet>,
    query: Query<&RigidBodyHandleComponent>,
) {
    if runstate.gamestate.is(GameState::Game) {
        if let Ok(body_handle) = query.get::<RigidBodyHandleComponent>(runstate.player.unwrap()) {
            let elapsed = time.delta_seconds;
            let mut body = bodies.get_mut(body_handle.handle()).unwrap();
            body.angvel = body.angvel * 0.1f32.powf(elapsed);
            body.linvel = body.linvel * 0.8f32.powf(elapsed);
        }
    }
}

pub fn ship_cannon_system(time: Res<Time>, mut ship: Query<Mut<Ship>>) {
    for mut ship in &mut ship.iter() {
        ship.cannon_timer.tick(time.delta_seconds);
    }
}

pub fn user_input_system(
    commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    audio_output: Res<AudioOutput>,
    mut runstate: ResMut<RunState>,
    input: Res<Input<KeyCode>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
    mut bodies: ResMut<RigidBodySet>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    query: Query<(&RigidBodyHandleComponent, Mut<Ship>)>,
) {
    if input.just_pressed(KeyCode::Back) {
        runstate.gamestate.transit_to(GameState::StartMenu);
    }
    if runstate.gamestate.is(GameState::Game) {
        let player = runstate.player.unwrap();
        let mut rotation = 0;
        let mut thrust = 0;
        if input.pressed(KeyCode::W) {
            thrust += 1
        }
        if input.pressed(KeyCode::A) {
            rotation += 1
        }
        if input.pressed(KeyCode::D) {
            rotation -= 1
        }
        if rotation != 0 || thrust != 0 {
            if let Ok(body_handle) = query.get::<RigidBodyHandleComponent>(player) {
                let mut body = bodies.get_mut(body_handle.handle()).unwrap();
                let ship = query.get::<Ship>(player).unwrap();
                if rotation != 0 {
                    let rotation = rotation as f32 * ship.rotation_speed;
                    body.wake_up();
                    body.apply_torque_impulse(rotation);
                }
                if thrust != 0 {
                    let force = body.position.rotation.transform_vector(&Vector2::y())
                        * thrust as f32
                        * ship.thrust;
                    body.wake_up();
                    body.apply_force(force);
                }
            }
        }
        if input.pressed(KeyCode::Space) {
            if let Ok(mut ship) = query.get_mut::<Ship>(player) {
                if ship.cannon_timer.finished {
                    if let Ok(body_handle) = query.get::<RigidBodyHandleComponent>(player) {
                        let body = bodies.get(body_handle.handle()).unwrap();
                        spawn_laser(commands, body, asset_server, materials, audio_output);
                    }
                    ship.cannon_timer.reset();
                }
            }
        }
        if input.just_pressed(KeyCode::Escape) {
            runstate.gamestate.transit_to(GameState::Pause);
            rapier_configuration.active = false;
        }
    } else if runstate.gamestate.is(GameState::StartMenu) {
        if input.just_pressed(KeyCode::Return) {
            runstate.gamestate.transit_to(GameState::Game);
        }
        if input.just_pressed(KeyCode::Escape) {
            app_exit_events.send(AppExit);
        }
    } else if runstate.gamestate.is(GameState::GameOver) {
        if input.just_pressed(KeyCode::Return) {
            runstate.gamestate.transit_to(GameState::StartMenu);
        }
        if input.just_pressed(KeyCode::Escape) {
            app_exit_events.send(AppExit);
        }
    } else if runstate.gamestate.is(GameState::Pause) {
        if input.just_pressed(KeyCode::Escape) {
            runstate.gamestate.transit_to(GameState::Game);
            rapier_configuration.active = true;
        }
    }
}
