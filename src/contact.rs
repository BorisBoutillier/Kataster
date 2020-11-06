use bevy::prelude::*;
use bevy_contrib_bobox::BodyHandleToEntity;
use bevy_rapier2d::{
    physics::{EventQueue, RigidBodyHandleComponent},
    rapier::{
        dynamics::RigidBodySet,
        geometry::{ContactEvent, Proximity},
    },
};

use super::arena::*;
use super::components::*;
use super::state::*;
use rand::{thread_rng, Rng};
enum Contacts {
    ShipAsteroid(Entity, Entity),
    LaserAsteroid(Entity, Entity),
}

pub fn contact_system(
    mut commands: Commands,
    mut asteroid_spawn_events: ResMut<Events<AsteroidSpawnEvent>>,
    mut explosion_spawn_events: ResMut<Events<ExplosionSpawnEvent>>,
    mut runstate: ResMut<RunState>,
    events: Res<EventQueue>,
    bh_to_e: Res<BodyHandleToEntity>,
    bodies: ResMut<RigidBodySet>,
    damages: Query<&Damage>,
    mut ships: Query<Mut<Ship>>,
    lasers: Query<&Laser>,
    asteroids: Query<&Asteroid>,
    handles: Query<&RigidBodyHandleComponent>,
) {
    if runstate.gamestate.is(GameState::Game) {
        let mut contacts = vec![];
        while let Ok(contact_event) = events.contact_events.pop() {
            match contact_event {
                ContactEvent::Started(h1, h2) => {
                    let e1 = *(bh_to_e.0.get(&h1).unwrap());
                    let e2 = *(bh_to_e.0.get(&h2).unwrap());
                    if ships.get_component::<Ship>(e1).is_ok()
                        && damages.get_component::<Damage>(e2).is_ok()
                    {
                        contacts.push(Contacts::ShipAsteroid(e1, e2));
                    } else if ships.get_component::<Ship>(e2).is_ok()
                        && damages.get_component::<Damage>(e1).is_ok()
                    {
                        contacts.push(Contacts::ShipAsteroid(e2, e1));
                    }
                }
                _ => (),
            };
        }
        while let Ok(proximity_event) = events.proximity_events.pop() {
            if proximity_event.new_status == Proximity::Intersecting {
                let e1 = *(bh_to_e.0.get(&proximity_event.collider1).unwrap());
                let e2 = *(bh_to_e.0.get(&proximity_event.collider2).unwrap());
                if asteroids.get_component::<Asteroid>(e2).is_ok()
                    && lasers.get_component::<Laser>(e1).is_ok()
                {
                    contacts.push(Contacts::LaserAsteroid(e1, e2));
                } else if asteroids.get_component::<Asteroid>(e1).is_ok()
                    && lasers.get_component::<Laser>(e2).is_ok()
                {
                    contacts.push(Contacts::LaserAsteroid(e2, e1));
                }
            }
        }
        for contact in contacts.into_iter() {
            match contact {
                Contacts::LaserAsteroid(e1, e2) => {
                    let laser_handle = handles
                        .get_component::<RigidBodyHandleComponent>(e1)
                        .unwrap()
                        .handle();
                    let asteroid = asteroids.get_component::<Asteroid>(e2).unwrap();
                    let asteroid_handle = handles
                        .get_component::<RigidBodyHandleComponent>(e2)
                        .unwrap()
                        .handle();
                    runstate.score = runstate.score.and_then(|score| {
                        Some(
                            score
                                + match asteroid.size {
                                    AsteroidSize::Small => 40,
                                    AsteroidSize::Medium => 20,
                                    AsteroidSize::Big => 10,
                                },
                        )
                    });
                    {
                        let laser_body = bodies.get(laser_handle).unwrap();
                        let asteroid_body = bodies.get(asteroid_handle).unwrap();

                        explosion_spawn_events.send(ExplosionSpawnEvent {
                            kind: ExplosionKind::LaserOnAsteroid,
                            x: laser_body.position.translation.x,
                            y: laser_body.position.translation.y,
                        });
                        if asteroid.size != AsteroidSize::Small {
                            let (size, radius) = match asteroid.size {
                                AsteroidSize::Big => (AsteroidSize::Medium, 5.0),
                                AsteroidSize::Medium => (AsteroidSize::Small, 2.0),
                                _ => panic!(),
                            };
                            let mut rng = thread_rng();
                            for _ in 0..rng.gen_range(1, 4) {
                                let x = asteroid_body.position.translation.x
                                    + rng.gen_range(-radius, radius);
                                let y = asteroid_body.position.translation.y
                                    + rng.gen_range(-radius, radius);
                                let vx = rng.gen_range(-ARENA_WIDTH / radius, ARENA_WIDTH / radius);
                                let vy =
                                    rng.gen_range(-ARENA_HEIGHT / radius, ARENA_HEIGHT / radius);
                                asteroid_spawn_events.send(AsteroidSpawnEvent {
                                    size,
                                    x,
                                    y,
                                    vx,
                                    vy,
                                    angvel: asteroid_body.angvel,
                                });
                            }
                        }
                    }
                    commands.despawn(e1);
                    commands.despawn(e2);
                }
                Contacts::ShipAsteroid(e1, e2) => {
                    let player_body = bodies
                        .get(
                            handles
                                .get_component::<RigidBodyHandleComponent>(e1)
                                .unwrap()
                                .handle(),
                        )
                        .unwrap();
                    let mut ship = ships.get_component_mut::<Ship>(e1).unwrap();
                    let damage = damages.get_component::<Damage>(e2).unwrap();
                    ship.life -= damage.value;
                    if ship.life <= 0 {
                        explosion_spawn_events.send(ExplosionSpawnEvent {
                            kind: ExplosionKind::ShipDead,
                            x: player_body.position.translation.x,
                            y: player_body.position.translation.y,
                        });
                        commands.despawn(e1);
                        runstate.gamestate.transit_to(GameState::GameOver);
                    } else {
                        explosion_spawn_events.send(ExplosionSpawnEvent {
                            kind: ExplosionKind::ShipContact,
                            x: player_body.position.translation.x,
                            y: player_body.position.translation.y,
                        });
                    }
                }
            }
        }
    }
}
