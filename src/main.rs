use bevy::prelude::*;
use bevy_rapier2d::{
    na::Vector2,
    physics::{Gravity, RapierPhysicsPlugin},
};

use std::collections::HashMap;

mod arena;
mod components;
mod contact;
mod explosion;
mod laser;
mod player;
mod utils;

use arena::*;
use components::*;
use contact::*;
use explosion::*;
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
        .add_resource(ClearColor(Color::rgb_u8(5, 5, 10)))
        .add_resource(BodyHandleToEntity(HashMap::new()))
        .add_event::<AsteroidSpawnEvent>()
        .add_event::<ExplosionSpawnEvent>()
        .add_plugin(RapierPhysicsPlugin)
        .add_default_plugins()
        .add_resource(Gravity(Vector2::zeros()))
        .add_startup_system(setup.system())
        .add_startup_system(spawn_player.system())
        .add_stage_after(stage::POST_UPDATE, "HANDLE_CONTACT")
        .add_stage_after("HANDLE_CONTACT", "HANDLE_EXPLOSION")
        .add_system(arena_spawn_system.system())
        .add_system(body_to_entity_system.system())
        .add_system(position_system.system())
        .add_system(user_input_system.system())
        .add_system(player_dampening_system.system())
        .add_system(despawn_laser_system.system())
        .add_system(handle_explosion.system())
        .add_system_to_stage(stage::POST_UPDATE, contact_system.system())
        .add_system_to_stage("HANDLE_CONTACT", spawn_asteroid_system.system())
        .add_system_to_stage("HANDLE_EXPLOSION", spawn_explosion.system())
        .run();
}
