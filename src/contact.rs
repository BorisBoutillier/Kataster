use bevy::prelude::*;
use bevy_rapier2d::{
    physics::EventQueue,
    physics::RigidBodyHandleComponent,
    rapier::{
        dynamics::{RigidBody, RigidBodySet},
        geometry::Proximity,
        ncollide::narrow_phase::ContactEvent,
    },
};
use std::ops::{Deref, DerefMut};

use super::components::*;
use super::utils::*;

pub fn contact_system(
    mut commands: Commands,
    events: Res<EventQueue>,
    mut bodies: ResMut<RigidBodySet>,
    h_to_e: Res<BodyHandleToEntity>,
    damages: Query<&Damage>,
    ships: Query<Mut<Ship>>,
    lasers: Query<Mut<Laser>>,
    asteroids: Query<Mut<Asteroid>>,
    handles: Query<&RigidBodyHandleComponent>,
) {
    while let Ok(contact_event) = events.contact_events.pop() {
        match contact_event {
            ContactEvent::Started(h1, h2) => {
                let e1 = *(h_to_e.0.get(&h1).unwrap());
                let e2 = *(h_to_e.0.get(&h2).unwrap());
                if let Ok(mut ship) = ships.get_mut::<Ship>(e1) {
                    if let Ok(damage) = damages.get::<Damage>(e2) {
                        player_damaged(ship.deref_mut(), damage.deref());
                    }
                }
                if let Ok(mut ship) = ships.get_mut::<Ship>(e2) {
                    if let Ok(damage) = damages.get::<Damage>(e1) {
                        player_damaged(ship.deref_mut(), damage.deref());
                    }
                }
            }
            _ => (),
        };
    }
    while let Ok(proximity_event) = events.proximity_events.pop() {
        if proximity_event.new_status == Proximity::Intersecting {
            let e1 = *(h_to_e.0.get(&proximity_event.collider1).unwrap());
            let e2 = *(h_to_e.0.get(&proximity_event.collider2).unwrap());
            if let Ok(asteroid) = asteroids.get::<Asteroid>(e2) {
                if let Ok(laser) = lasers.get::<Laser>(e1) {
                    let asteroid_body = bodies
                        .get(
                            handles
                                .get::<RigidBodyHandleComponent>(e2)
                                .unwrap()
                                .handle(),
                        )
                        .unwrap();
                    asteroid_damaged(&mut commands, asteroid.deref(), &asteroid_body, e2, e1);
                }
            }
            if let Ok(asteroid) = asteroids.get::<Asteroid>(e1) {
                if let Ok(laser) = lasers.get::<Laser>(e2) {
                    let asteroid_body = bodies
                        .get(
                            handles
                                .get::<RigidBodyHandleComponent>(e1)
                                .unwrap()
                                .handle(),
                        )
                        .unwrap();
                    asteroid_damaged(&mut commands, asteroid.deref(), &asteroid_body, e1, e2);
                }
            }
        }
    }
}
fn player_damaged(ship: &mut Ship, damage: &Damage) {
    ship.life -= damage.value;
    if ship.life <= 0 {
        println!("Player DEAD")
    } else {
        println!("Player contact Life: {}", ship.life)
    }
}
fn asteroid_damaged(
    mut commands: &mut Commands,
    asteroid: &Asteroid,
    asteroid_body: &RigidBody,
    asteroid_entity: Entity,
    laser_entity: Entity,
) {
    commands.despawn(asteroid_entity);
    commands.despawn(laser_entity);
}
