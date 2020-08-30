use bevy::prelude::*;
pub struct Player(pub Entity);

pub struct Ship {
    /// Ship rotation speed in rad/s
    pub rotation_speed: f32,
    /// Ship thrust N
    pub thrust: f32,
    /// Ship life points
    pub life: u32,
}

pub enum AsteroidSize {
    Big,
    Medium,
    Small,
}
pub struct Asteroid {
    pub size: AsteroidSize,
}
pub struct Laser {
    pub despawn_timer: Timer,
}
pub struct Damage {
    pub value: u32,
}
