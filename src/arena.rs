use crate::prelude::*;

pub const ARENA_WIDTH: f32 = 1280.0;
pub const ARENA_HEIGHT: f32 = 800.0;

#[derive(Debug, Resource)]
pub struct Arena {
    pub asteroid_spawn_timer: Timer,
    pub score: u32,
}

#[derive(PhysicsLayer)]
pub enum GameLayer {
    Player,
    Laser,
    Asteroid,
}

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Setup), spawn_arena)
            .add_systems(
                OnEnter(GameState::Running),
                |mut physics_time: ResMut<Time<Physics>>| {
                    physics_time.unpause();
                },
            )
            .add_systems(
                OnEnter(GameState::Paused),
                |mut physics_time: ResMut<Time<Physics>>| {
                    physics_time.pause();
                },
            )
            .add_systems(Update, movement.run_if(in_state(GameState::Running)));
    }
}

fn spawn_arena(mut commands: Commands) {
    commands.insert_resource(Arena {
        asteroid_spawn_timer: Timer::from_seconds(5.0, TimerMode::Once),
        score: 0,
    });

    // Physics configuration without gravity
    commands.insert_resource(Gravity::ZERO);
}

fn movement(mut query: Query<(&LinearVelocity, &mut Position)>) {
    for (linvel, mut position) in query.iter_mut() {
        let mut x = position.x;
        let mut y = position.y;
        let mut updated = false;
        // Wrap around screen edges
        let half_width = ARENA_WIDTH / 2.0;
        let half_height = ARENA_HEIGHT / 2.0;
        if x < -half_width && linvel.x < 0.0 {
            x = half_width;
            updated = true;
        } else if x > half_width && linvel.x > 0.0 {
            x = -half_width;
            updated = true;
        }
        if y < -half_height && linvel.y < 0.0 {
            y = half_height;
            updated = true;
        } else if y > half_height && linvel.y > 0.0 {
            y = -half_height;
            updated = true;
        }
        if updated {
            position.x = x;
            position.y = y;
        }
    }
}
