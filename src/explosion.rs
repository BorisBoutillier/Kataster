use super::components::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct SpawnExplosionState {
    event_reader: EventReader<ExplosionSpawnEvent>,
}

pub struct Explosion {
    timer: Timer,
    start_scale: f32,
    end_scale: f32,
}
pub fn spawn_explosion(
    mut commands: Commands,
    mut state: Local<SpawnExplosionState>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    audio_output: Res<AudioOutput>,
    events: Res<Events<ExplosionSpawnEvent>>,
) {
    for event in state.event_reader.iter(&events) {
        let (texture_name, sound_name, start_scale, end_scale, duration) = match event.kind {
            ExplosionKind::Ship => (
                "assets/explosion01.png",
                "assets/Explosion_ship.mp3",
                0.1 / 15.0,
                0.5 / 15.0,
                1.5,
            ),
            ExplosionKind::LaserOnAsteroid => (
                "assets/flash00.png",
                "assets/Explosion.mp3",
                0.1 / 15.0,
                0.15 / 15.0,
                0.5,
            ),
        };
        let texture_handle = asset_server.load(texture_name).unwrap();
        commands
            .spawn(SpriteComponents {
                translation: Translation::new(event.x, event.y, 2.0),
                material: materials.add(texture_handle.into()),
                scale: Scale(start_scale),
                ..Default::default()
            })
            .with(Explosion {
                timer: Timer::from_seconds(duration, false),
                start_scale,
                end_scale,
            });
        let sound = asset_server.load(sound_name).unwrap();
        audio_output.play(sound);
    }
}

pub fn handle_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, Mut<Scale>, Mut<Explosion>)>,
) {
    let elapsed = time.delta_seconds;
    for (entity, mut scale, mut explosion) in &mut query.iter() {
        explosion.timer.tick(elapsed);
        if explosion.timer.finished {
            commands.despawn(entity);
        } else {
            scale.0 = explosion.start_scale
                + (explosion.end_scale - explosion.start_scale)
                    * (explosion.timer.elapsed / explosion.timer.duration);
        }
    }
}
