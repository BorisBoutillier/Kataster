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
) {
    for CollisionStarted(e1, e2) in collision_events.read() {
        let (e1_is_ship, e1_is_laser, e1_is_asteroid) = query.get(*e1).unwrap();
        let (e2_is_ship, e2_is_laser, e2_is_asteroid) = query.get(*e2).unwrap();
        if e1_is_ship && e2_is_asteroid {
            ship_asteroid_contact_events.send(ShipAsteroidContactEvent { ship: *e1 });
        }
        if e2_is_ship && e1_is_asteroid {
            ship_asteroid_contact_events.send(ShipAsteroidContactEvent { ship: *e2 });
        }
        if e1_is_asteroid && e2_is_laser {
            laser_asteroid_contact_events.send(LaserAsteroidContactEvent {
                asteroid: *e1,
                laser: *e2,
            });
        }
        if e2_is_asteroid && e1_is_laser {
            laser_asteroid_contact_events.send(LaserAsteroidContactEvent {
                asteroid: *e2,
                laser: *e1,
            });
        }
    }
}
