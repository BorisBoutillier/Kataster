use crate::prelude::*;

#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct ContactLabel;

pub struct ContactPlugin;

impl Plugin for ContactPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(contact_system.label(ContactLabel)),
        );
    }
}

pub fn contact_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut ship_asteroid_contact_events: EventWriter<ShipAsteroidContactEvent>,
    mut laser_asteroid_contact_events: EventWriter<LaserAsteroidContactEvent>,
    ships: Query<&Ship>,
    lasers: Query<&Laser>,
    asteroids: Query<&Asteroid>,
) {
    for event in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, _flags) = event {
            if ships.get(*e1).is_ok() && asteroids.get(*e2).is_ok() {
                ship_asteroid_contact_events.send(ShipAsteroidContactEvent {
                    ship: *e1,
                    asteroid: *e2,
                });
            }
            if ships.get(*e2).is_ok() && asteroids.get(*e1).is_ok() {
                ship_asteroid_contact_events.send(ShipAsteroidContactEvent {
                    ship: *e2,
                    asteroid: *e1,
                });
            }
            if asteroids.get_component::<Asteroid>(*e1).is_ok()
                && lasers.get_component::<Laser>(*e2).is_ok()
            {
                laser_asteroid_contact_events.send(LaserAsteroidContactEvent {
                    asteroid: *e1,
                    laser: *e2,
                });
            }
            if asteroids.get_component::<Asteroid>(*e2).is_ok()
                && lasers.get_component::<Laser>(*e1).is_ok()
            {
                laser_asteroid_contact_events.send(LaserAsteroidContactEvent {
                    asteroid: *e2,
                    laser: *e1,
                });
            }
        }
    }
}
