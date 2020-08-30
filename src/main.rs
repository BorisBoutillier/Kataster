use bevy::{prelude::*, render::pass::ClearColor};
use bevy_rapier2d::{
    na::Vector2,
    physics::{Gravity, RapierPhysicsPlugin},
};

use std::collections::HashMap;

mod arena;
mod components;
mod contact;
mod laser;
mod player;
mod utils;

use arena::*;
use contact::*;
use laser::*;
use player::*;
use utils::*;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 2 })
        .add_resource(WindowDescriptor {
            title: "Kataster".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.02, 0.02, 0.04)))
        .add_default_plugins()
        .add_plugin(RapierPhysicsPlugin)
        .add_resource(Gravity(Vector2::zeros()))
        .add_startup_system(setup.system())
        .add_startup_system(spawn_player.system())
        .add_system(spawn_random_asteroid.system())
        .add_system(position_system.system())
        .add_system(user_input_system.system())
        .add_system(player_dampening_system.system())
        .add_system(body_to_entity_system.system())
        .add_system(despawn_laser_system.system())
        .add_system_to_stage(stage::POST_UPDATE, contact_system.system())
        .add_resource(BodyHandleToEntity(HashMap::new()))
        .run();
}
