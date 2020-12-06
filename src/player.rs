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
    let texture_handle = asset_server.load("playerShip2_red.png");
    commands
        .spawn(SpriteComponents {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -5.0),
                scale: Vec3::splat(1.0 / 37.0),
                ..Default::default()
            },
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with(Ship {
            rotation_speed: 0.3,
            thrust: 60.0,
            life: START_LIFE,
            cannon_timer: Timer::from_seconds(0.2, false),
        })
        .with(ForStates {
            states: vec![GameState::Game, GameState::Pause, GameState::GameOver],
        });
    let player_entity = commands.current_entity().unwrap();
    let body = RigidBodyBuilder::new_dynamic().user_data(player_entity.to_bits() as u128);
    let collider = ColliderBuilder::ball(1.0);
    // The triangle Collider does not compute mass
    //let collider = ColliderBuilder::triangle(
    //    Point::new(1.0, -0.5),
    //    Point::new(0.0, 0.8),
    //    Point::new(-1.0, -0.5),
    //);
    commands.insert(player_entity, (body, collider));
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
        if let Ok(body_handle) =
            query.get_component::<RigidBodyHandleComponent>(runstate.player.unwrap())
        {
            let elapsed = time.delta_seconds;
            let body = bodies.get_mut(body_handle.handle()).unwrap();
            body.set_angvel(body.angvel() * 0.1f32.powf(elapsed), false);
            body.set_linvel(body.linvel() * 0.8f32.powf(elapsed), false);
        }
    }
}

pub fn ship_cannon_system(time: Res<Time>, mut ship: Query<Mut<Ship>>) {
    for mut ship in ship.iter_mut() {
        ship.cannon_timer.tick(time.delta_seconds);
    }
}

pub fn user_input_system(
    commands: Commands,
    audio: Res<Audio>,
    mut runstate: ResMut<RunState>,
    input: Res<Input<KeyCode>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
    mut bodies: ResMut<RigidBodySet>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    mut query: Query<(&RigidBodyHandleComponent, Mut<Ship>)>,
) {
    if !runstate.gamestate.is(GameState::StartMenu) {
        if input.just_pressed(KeyCode::Back) {
            runstate.gamestate.transit_to(GameState::StartMenu);
        }
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
            if let Ok(body_handle) = query.get_component::<RigidBodyHandleComponent>(player) {
                let body = bodies.get_mut(body_handle.handle()).unwrap();
                let ship = query.get_component::<Ship>(player).unwrap();
                if rotation != 0 {
                    let rotation = rotation as f32 * ship.rotation_speed;
                    body.apply_torque_impulse(rotation, true);
                }
                if thrust != 0 {
                    let force = body.position().rotation.transform_vector(&Vector2::y())
                        * thrust as f32
                        * ship.thrust;
                    body.apply_force(force, true);
                }
            }
        }
        if input.pressed(KeyCode::Space) {
            if let Ok((body_handle, mut ship)) = query.get_mut(player) {
                if ship.cannon_timer.finished {
                    let body = bodies.get(body_handle.handle()).unwrap();
                    spawn_laser(commands, body, &runstate, audio);
                    ship.cannon_timer.reset();
                }
            }
        }
        if input.just_pressed(KeyCode::Escape) {
            runstate.gamestate.transit_to(GameState::Pause);
            rapier_configuration.query_pipeline_active = false;
            rapier_configuration.physics_pipeline_active = false;
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
            rapier_configuration.query_pipeline_active = true;
            rapier_configuration.physics_pipeline_active = true;
        }
    }
}
