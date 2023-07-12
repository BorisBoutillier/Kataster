use crate::prelude::*;

pub const ARENA_WIDTH: f32 = 1280.0;
pub const ARENA_HEIGHT: f32 = 800.0;

#[derive(Debug, Resource)]
pub struct Arena {
    pub asteroid_spawn_timer: Timer,
    pub score: u32,
}

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameCreate), spawn_arena)
            .add_systems(Update, movement.run_if(in_state(AppState::GameRunning)));
    }
}

fn spawn_arena(mut commands: Commands, mut rapier_configuration: ResMut<RapierConfiguration>) {
    commands.insert_resource(Arena {
        asteroid_spawn_timer: Timer::from_seconds(5.0, TimerMode::Once),
        score: 0,
    });

    // Rapier configuration without gravity
    rapier_configuration.gravity = Vec2::ZERO;
}

fn movement(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        let mut x = transform.translation.x;
        let mut y = transform.translation.y;
        let mut updated = false;
        // Wrap around screen edges
        let half_width = ARENA_WIDTH / 2.0;
        let half_height = ARENA_HEIGHT / 2.0;
        if x < -half_width && velocity.linvel.x < 0.0 {
            x = half_width;
            updated = true;
        } else if x > half_width && velocity.linvel.x > 0.0 {
            x = -half_width;
            updated = true;
        }
        if y < -half_height && velocity.linvel.y < 0.0 {
            y = half_height;
            updated = true;
        } else if y > half_height && velocity.linvel.y > 0.0 {
            y = -half_height;
            updated = true;
        }
        if updated {
            transform.translation.x = x;
            transform.translation.y = y;
        }
    }
}
