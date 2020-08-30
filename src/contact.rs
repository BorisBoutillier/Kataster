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

use super::arena::*;
use super::components::*;
use super::utils::*;
use rand::{thread_rng, Rng};

pub fn contact_system(
    mut commands: Commands,
    mut asteroid_spawn_events: ResMut<Events<AsteroidSpawnEvent>>,
    mut explosion_spawn_events: ResMut<Events<ExplosionSpawnEvent>>,
    events: Res<EventQueue>,
    bodies: ResMut<RigidBodySet>,
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
                        let body = bodies
                            .get(
                                handles
                                    .get::<RigidBodyHandleComponent>(e1)
                                    .unwrap()
                                    .handle(),
                            )
                            .unwrap();
                        player_damaged(
                            &mut commands,
                            &mut explosion_spawn_events,
                            e1,
                            body,
                            ship.deref_mut(),
                            damage.deref(),
                        );
                    }
                }
                if let Ok(mut ship) = ships.get_mut::<Ship>(e2) {
                    if let Ok(damage) = damages.get::<Damage>(e1) {
                        let body = bodies
                            .get(
                                handles
                                    .get::<RigidBodyHandleComponent>(e2)
                                    .unwrap()
                                    .handle(),
                            )
                            .unwrap();
                        player_damaged(
                            &mut commands,
                            &mut explosion_spawn_events,
                            e2,
                            body,
                            ship.deref_mut(),
                            damage.deref(),
                        );
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
                if let Ok(_) = lasers.get::<Laser>(e1) {
                    let asteroid_body = bodies
                        .get(
                            handles
                                .get::<RigidBodyHandleComponent>(e2)
                                .unwrap()
                                .handle(),
                        )
                        .unwrap();
                    let laser_body = bodies
                        .get(
                            handles
                                .get::<RigidBodyHandleComponent>(e1)
                                .unwrap()
                                .handle(),
                        )
                        .unwrap();
                    asteroid_damaged(
                        &mut commands,
                        &mut asteroid_spawn_events,
                        &mut explosion_spawn_events,
                        asteroid.deref(),
                        &asteroid_body,
                        e1,
                        &laser_body,
                        e2,
                    );
                }
            }
            if let Ok(asteroid) = asteroids.get::<Asteroid>(e1) {
                if let Ok(_) = lasers.get::<Laser>(e2) {
                    let asteroid_body = bodies
                        .get(
                            handles
                                .get::<RigidBodyHandleComponent>(e1)
                                .unwrap()
                                .handle(),
                        )
                        .unwrap();
                    let laser_body = bodies
                        .get(
                            handles
                                .get::<RigidBodyHandleComponent>(e2)
                                .unwrap()
                                .handle(),
                        )
                        .unwrap();
                    asteroid_damaged(
                        &mut commands,
                        &mut asteroid_spawn_events,
                        &mut explosion_spawn_events,
                        asteroid.deref(),
                        &asteroid_body,
                        e1,
                        &laser_body,
                        e2,
                    );
                }
            }
        }
    }
}
fn player_damaged(
    commands: &mut Commands,
    explosion_spawn_events: &mut ResMut<Events<ExplosionSpawnEvent>>,
    player_entity: Entity,
    player_body: &RigidBody,
    ship: &mut Ship,
    damage: &Damage,
) {
    ship.life -= damage.value;
    explosion_spawn_events.send(ExplosionSpawnEvent {
        kind: ExplosionKind::Ship,
        x: player_body.position.translation.x,
        y: player_body.position.translation.y,
    });
    if ship.life <= 0 {
        commands.despawn(player_entity);
    } else {
    }
}
fn asteroid_damaged(
    commands: &mut Commands,
    asteroid_spawn_events: &mut ResMut<Events<AsteroidSpawnEvent>>,
    explosion_spawn_events: &mut ResMut<Events<ExplosionSpawnEvent>>,
    asteroid: &Asteroid,
    asteroid_body: &RigidBody,
    asteroid_entity: Entity,
    laser_body: &RigidBody,
    laser_entity: Entity,
) {
    commands.despawn(asteroid_entity);
    commands.despawn(laser_entity);
    explosion_spawn_events.send(ExplosionSpawnEvent {
        kind: ExplosionKind::LaserOnAsteroid,
        x: laser_body.position.translation.x,
        y: laser_body.position.translation.y,
    });
    if asteroid.size != AsteroidSize::Small {
        let (new_size, radius) = match asteroid.size {
            AsteroidSize::Big => (AsteroidSize::Medium, 5.0),
            AsteroidSize::Medium => (AsteroidSize::Small, 2.0),
            _ => panic!(),
        };
        let mut rng = thread_rng();
        for _ in 0..rng.gen_range(1, 4) {
            let x = asteroid_body.position.translation.x + rng.gen_range(-radius, radius);
            let y = asteroid_body.position.translation.y + rng.gen_range(-radius, radius);
            let vx = rng.gen_range(-ARENA_WIDTH / radius, ARENA_WIDTH / radius);
            let vy = rng.gen_range(-ARENA_HEIGHT / radius, ARENA_HEIGHT / radius);
            asteroid_spawn_events.send(AsteroidSpawnEvent {
                size: new_size,
                x,
                y,
                vx,
                vy,
                angvel: asteroid_body.angvel,
            });
        }
    }
}
