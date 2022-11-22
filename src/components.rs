use bevy::prelude::*;

pub enum ExplosionKind {
    ShipDead,
    ShipContact,
    LaserOnAsteroid,
}
pub struct ExplosionSpawnEvent {
    pub kind: ExplosionKind,
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Damage {
    pub value: u32,
}
