use crate::prelude::*;

pub enum ExplosionKind {
    ShipDead,
    ShipContact,
    LaserOnAsteroid,
}
#[derive(Event)]
pub struct SpawnExplosionEvent {
    pub kind: ExplosionKind,
    pub x: f32,
    pub y: f32,
}

/// Main component for an Explosion FX entity.
/// The FX is a simple animation of the scaling of the associated sprite
/// from a `start_scale` to an `end_scale` through the lifetime of a `timer`.
#[derive(Component)]
pub struct Explosion {
    timer: Timer,
    start_scale: f32,
    end_scale: f32,
}

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnExplosionEvent>()
            .add_systems(Update, (animate_explosion, catch_explosion_event));
    }
}

fn catch_explosion_event(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnExplosionEvent>,
    handles: Res<SpriteAssets>,
    audios: Res<AudioAssets>,
) {
    for event in event_reader.read() {
        let (texture, sound, start_size, end_scale, duration) = match event.kind {
            ExplosionKind::ShipDead => (
                handles.ship_explosion.clone(),
                audios.ship_explosion.clone(),
                Vec2::new(42., 39.),
                5.,
                2.,
            ),
            ExplosionKind::ShipContact => (
                handles.ship_contact.clone(),
                audios.ship_contact.clone(),
                Vec2::new(42., 39.),
                2.,
                1.,
            ),
            ExplosionKind::LaserOnAsteroid => (
                handles.asteroid_explosion.clone(),
                audios.asteroid_explosion.clone(),
                Vec2::new(36., 32.),
                1.5,
                1.,
            ),
        };
        commands.spawn((
            Sprite {
                image: texture,
                custom_size: Some(start_size),
                ..default()
            },
            Transform::from_translation(Vec3::new(event.x, event.y, 3.0)),
            Explosion {
                timer: Timer::from_seconds(duration, TimerMode::Once),
                start_scale: 1.,
                end_scale,
            },
            StateScoped(AppState::Game),
            AudioPlayer(sound),
        ));
    }
}

/// System that handles animation of the explosions.
///
/// For each explosion, tick its timer and either update the scaling or,
/// when finished, despawn the whole entity.
fn animate_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Explosion)>,
    game_state: Res<State<GameState>>,
) {
    // We want the explosions to be 'stalled' in the paused state.
    // So we don't tick any of the timers.
    if game_state.get() != &GameState::Paused {
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
}
