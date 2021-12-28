use bevy::prelude::*;

#[derive(Component)]
pub struct Ship {
    /// Ship rotation speed in rad/s
    pub rotation_speed: f32,
    /// Ship thrust N
    pub thrust: f32,
    /// Ship life points
    pub life: u32,
    /// Cannon auto-fire timer
    pub cannon_timer: Timer,
}

#[derive(Component)]
pub struct UiScore {}
#[derive(Component)]
pub struct UiLife {
    pub min: u32,
}

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

pub struct AsteroidSpawnEvent {
    pub size: AsteroidSize,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub angvel: f32,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AsteroidSize {
    Big,
    Medium,
    Small,
}
#[derive(Component)]
pub struct Asteroid {
    pub size: AsteroidSize,
}
#[derive(Component)]
pub struct Laser {
    pub despawn_timer: Timer,
}
#[derive(Component)]
pub struct Damage {
    pub value: u32,
}
