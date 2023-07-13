use bevy::ecs::query::Has;

use crate::prelude::*;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub struct ContactSet;

pub struct ContactPlugin;

impl Plugin for ContactPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            contact_system
                .in_set(ContactSet)
                .run_if(in_state(AppState::GameRunning)),
        );
    }
}

fn contact_system(
    mut collision_events: EventReader<CollisionStarted>,
    mut ship_asteroid_contact_events: EventWriter<ShipAsteroidContactEvent>,
    mut laser_asteroid_contact_events: EventWriter<LaserAsteroidContactEvent>,
    query: Query<(Has<Ship>, Has<Laser>, Has<Asteroid>)>,
    ships: Query<&Ship>,
    lasers: Query<&Laser>,
    asteroids: Query<&Asteroid>,
) {
    if !collision_events.is_empty() {
        println!("------");
    }
    let mut count = 0;
    for CollisionStarted(e1, e2) in collision_events.read() {
        println!("Collision {:?} {:?}", e1, e2);
        //let (e1_is_ship,e1_is_laser,e1_is_asteroid) = query.get(*e1).unwrap();
        //let (e2_is_ship,e2_is_laser,e2_is_asteroid) = query.get(*e1).unwrap();
        if ships.get(*e1).is_ok() && asteroids.get(*e2).is_ok() {
            ship_asteroid_contact_events.send(ShipAsteroidContactEvent {
                ship: *e1,
                asteroid: *e2,
            });
            count += 1;
        }
        if ships.get(*e2).is_ok() && asteroids.get(*e1).is_ok() {
            ship_asteroid_contact_events.send(ShipAsteroidContactEvent {
                ship: *e2,
                asteroid: *e1,
            });
            count += 1;
        }
        if asteroids.get_component::<Asteroid>(*e1).is_ok()
            && lasers.get_component::<Laser>(*e2).is_ok()
        {
            laser_asteroid_contact_events.send(LaserAsteroidContactEvent {
                asteroid: *e1,
                laser: *e2,
            });
            count += 1;
        }
        if asteroids.get_component::<Asteroid>(*e2).is_ok()
            && lasers.get_component::<Laser>(*e1).is_ok()
        {
            laser_asteroid_contact_events.send(LaserAsteroidContactEvent {
                asteroid: *e2,
                laser: *e1,
            });
            count += 1;
        }
    }
    if count > 1 {
        panic!();
    }
}
