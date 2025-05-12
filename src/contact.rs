use bevy::ecs::query::Has;

use crate::prelude::*;

// An event that will be triggered whenever an entity receives damage.
// This game is simple so there is no need for damage types, values or source types.
#[derive(Event)]
pub struct Damage;

pub struct ContactPlugin;

impl Plugin for ContactPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, contact_system.run_if(in_state(GameState::Running)));
    }
}

fn contact_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionStarted>,
    mut explosion_spawn_events: EventWriter<SpawnExplosionEvent>,
    query: Query<(Has<Ship>, Has<Laser>, Has<Asteroid>)>,
    transforms: Query<&Transform>,
) {
    for CollisionStarted(e1, e2) in collision_events.read() {
        // First compute what collided what.
        let (e1_is_ship, e1_is_laser, e1_is_asteroid) = query.get(*e1).unwrap();
        let (e2_is_ship, e2_is_laser, e2_is_asteroid) = query.get(*e2).unwrap();
        let ship = match (e1_is_ship, e2_is_ship) {
            (true, _) => Some(e1),
            (_, true) => Some(e2),
            _ => None,
        };
        let asteroid = match (e1_is_asteroid, e2_is_asteroid) {
            (true, _) => Some(e1),
            (_, true) => Some(e2),
            _ => None,
        };
        let laser = match (e1_is_laser, e2_is_laser) {
            (true, _) => Some(e1),
            (_, true) => Some(e2),
            _ => None,
        };
        // Ship on Asteroid collision
        // The asteroid is unaffected, only the ship takes damage.
        // Possible explosion VFX is handled by the ship damage system.
        if let (Some(ship), Some(_)) = (ship, asteroid) {
            commands.trigger_targets(Damage, *ship);
        }
        // Laser on Asteroid collision
        // The asteroid is damaged and the laser despawned.
        // A LaserOnAsteroid explosion VFX is triggered. To simplify code
        // the VFX is triggered at the laser position and not at the exact contact position.
        if let (Some(laser), Some(asteroid)) = (laser, asteroid) {
            commands.trigger_targets(Damage, *asteroid);
            let laser_transform = transforms
                .get(*laser)
                .expect("Missing transform for the laser");
            explosion_spawn_events.write(SpawnExplosionEvent {
                kind: ExplosionKind::LaserOnAsteroid,
                x: laser_transform.translation.x,
                y: laser_transform.translation.y,
            });
            commands.entity(*laser).despawn();
        }
    }
}
