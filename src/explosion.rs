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
    for event in event_reader.iter() {
        let (texture, sound, start_size, end_scale, duration) = match event.kind {
            ExplosionKind::ShipDead => (
                handles.ship_explosion.clone(),
                audios.ship_explosion.clone(),
                Vec2::new(42., 39.),
                5.,
                1.5,
            ),
            ExplosionKind::ShipContact => (
                handles.ship_contact.clone(),
                audios.ship_contact.clone(),
                Vec2::new(42., 39.),
                2.,
                0.5,
            ),
            ExplosionKind::LaserOnAsteroid => (
                handles.asteroid_explosion.clone(),
                audios.asteroid_explosion.clone(),
                Vec2::new(36., 32.),
                1.5,
                0.5,
            ),
        };
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(start_size),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(event.x, event.y, 3.0),
                    ..default()
                },
                texture,
                ..default()
            },
            Explosion {
                timer: Timer::from_seconds(duration, TimerMode::Once),
                start_scale: 1.,
                end_scale,
            },
            ForState {
                states: AppState::ANY_GAME_STATE.to_vec(),
            },
            AudioBundle {
                source: sound,
                ..default()
            },
        ));
    }
}

fn animate_explosion(
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
