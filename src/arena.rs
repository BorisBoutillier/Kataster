use crate::prelude::*;

pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 800;
pub const CAMERA_SCALE: f32 = 1.;
pub const ARENA_WIDTH: f32 = WINDOW_WIDTH as f32 * CAMERA_SCALE;
pub const ARENA_HEIGHT: f32 = WINDOW_HEIGHT as f32 * CAMERA_SCALE;

#[derive(Debug)]
pub struct Arena {
    pub asteroid_spawn_timer: Timer,
}
pub fn setup_arena(mut runstate: ResMut<RunState>) {
    runstate.arena = Some(Arena {
        asteroid_spawn_timer: Timer::from_seconds(5.0, false),
    });
    runstate.score = Some(0);
}

pub fn position_system(mut query: Query<(&Velocity, &mut Transform)>) {
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
