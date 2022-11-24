use crate::prelude::*;

pub struct LaserDespawnEvent(pub Entity);
pub struct LaserSpawnEvent {
    // The full position (translation+rotation) of the laser to spawn
    pub transform: Transform,
    // The velocity of the entity emitting the laser
    pub velocity: Velocity,
}

#[derive(Component)]
pub struct Laser {
    pub despawn_timer: Timer,
}
pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LaserDespawnEvent>()
            .add_event::<LaserSpawnEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(laser_timeout_system)
                    .with_system(spawn_laser),
            )
            .add_system_to_stage(CoreStage::PostUpdate, laser_timeout_system);
    }
}

fn spawn_laser(
    mut commands: Commands,
    mut laser_spawn_events: EventReader<LaserSpawnEvent>,
    handles: Res<GameAssets>,
    audio: Res<Audio>,
) {
    for spawn_event in laser_spawn_events.iter() {
        let transform = spawn_event.transform;
        let velocity = Velocity::linear(
            (spawn_event.velocity.linvel * Vec2::Y)
                + (transform.rotation * Vec3::Y * 500.0).truncate(),
        );
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(5., 20.0)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(transform.translation.x, transform.translation.y, 2.0),
                    rotation: transform.rotation,
                    ..Default::default()
                },
                texture: handles.laser_texture.clone(),
                ..Default::default()
            },
            Laser {
                despawn_timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
            ForState {
                states: vec![AppState::Game],
            },
            RigidBody::Dynamic,
            Collider::cuboid(2.5, 10.0),
            velocity,
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
        ));
        audio.play(handles.laser_audio.clone());
    }
}

fn laser_timeout_system(
    mut commands: Commands,
    time: Res<Time>,
    gamestate: Res<State<AppGameState>>,
    mut query: Query<(Entity, &mut Laser)>,
) {
    if gamestate.current() == &AppGameState::Game {
        for (entity, mut laser) in query.iter_mut() {
            laser.despawn_timer.tick(time.delta());
            if laser.despawn_timer.finished() {
                commands.entity(entity).despawn();
            }
        }
    }
}
