use crate::prelude::*;

#[derive(Component)]
pub struct Explosion {
    timer: Timer,
    start_scale: f32,
    end_scale: f32,
}
pub fn spawn_explosion_event(
    mut commands: Commands,
    mut event_reader: EventReader<ExplosionSpawnEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for event in event_reader.iter() {
        let (texture_name, sound_name, start_size, end_scale, duration) = match event.kind {
            ExplosionKind::ShipDead => (
                "explosion01.png",
                "Explosion_ship.ogg",
                Vec2::new(42., 39.),
                5.,
                1.5,
            ),
            ExplosionKind::ShipContact => (
                "explosion01.png",
                "Explosion.ogg",
                Vec2::new(42., 39.),
                2.,
                0.5,
            ),
            ExplosionKind::LaserOnAsteroid => (
                "flash00.png",
                "Explosion.ogg",
                Vec2::new(36., 32.),
                1.5,
                0.5,
            ),
        };
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(start_size),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(event.x, event.y, -1.0),
                    ..Default::default()
                },
                texture: asset_server.load(texture_name),
                ..Default::default()
            })
            .insert(Explosion {
                timer: Timer::from_seconds(duration, false),
                start_scale: 1.,
                end_scale,
            })
            .insert(ForState {
                states: vec![AppState::Game],
            });
        let sound = asset_server.load(sound_name);
        audio.play(sound);
    }
}

pub fn handle_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Explosion)>,
) {
    let elapsed = time.delta();
    for (entity, mut transform, mut explosion) in query.iter_mut() {
        explosion.timer.tick(elapsed);
        if explosion.timer.finished() {
            commands.entity(entity).despawn();
        } else {
            transform.scale = Vec3::splat(
                explosion.start_scale
                    + (explosion.end_scale - explosion.start_scale)
                        * (explosion.timer.elapsed_secs()
                            / explosion.timer.duration().as_secs_f32()),
            );
        }
    }
}
