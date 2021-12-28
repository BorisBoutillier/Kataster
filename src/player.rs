use crate::prelude::*;
use bevy::app::AppExit;

pub const START_LIFE: u32 = 3;

pub fn spawn_player(
    mut commands: Commands,
    mut runstate: ResMut<RunState>,
    asset_server: Res<AssetServer>,
) {
    let mut player_entity_builder = commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -5.0),
            scale: Vec3::splat(1.0 / 37.0),
            ..Default::default()
        },
        texture: asset_server.load("playerShip2_red.png"),
        ..Default::default()
    });
    player_entity_builder
        .insert(Ship {
            rotation_speed: 3.0,
            thrust: 60.0,
            life: START_LIFE,
            cannon_timer: Timer::from_seconds(0.2, false),
        })
        .insert(ForState {
            states: vec![AppState::Game],
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere { radius: 1.0 })
        .insert(Acceleration::from_linear(Vec3::ZERO))
        .insert(Velocity::from_linear(Vec3::ZERO))
        .insert(
            CollisionLayers::none()
                .with_group(ArenaLayer::Player)
                .with_mask(ArenaLayer::World),
        );
    let player_entity = player_entity_builder.id();
    runstate.player = Some(player_entity);

    // Helper points to visualize some points in space for Collider
    //commands
    //    .spawn_bundle(SpriteComponents {
    //        translation: Translation::new(1.2, -1.0, 2.0),
    //        material: materials.add(texture_handle.into()),
    //        scale: Scale(0.001),
    //        ..Default::default()
    //    })
    //    .spawn_bundle(SpriteComponents {
    //        translation: Translation::new(0.0, 1.0, 2.0),
    //        material: materials.add(texture_handle.into()),
    //        scale: Scale(0.001),
    //        ..Default::default()
    //    })
    //    .spawn_bundle(SpriteComponents {
    //        translation: Translation::new(-1.2, -1.0, 2.0),
    //        material: materials.add(texture_handle.into()),
    //        scale: Scale(0.001),
    //        ..Default::default()
    //    });
}

pub fn player_dampening_system(
    time: Res<Time>,
    runstate: Res<RunState>,
    mut query: Query<&mut Velocity>,
) {
    if let Ok(mut velocity) = query.get_component_mut::<Velocity>(runstate.player.unwrap()) {
        let elapsed = time.delta_seconds();
        velocity.angular *= 0.1f32.powf(elapsed);
        velocity.linear *= 0.4f32.powf(elapsed);
    }
}

pub fn ship_cannon_system(time: Res<Time>, mut ship: Query<&mut Ship>) {
    for mut ship in ship.iter_mut() {
        ship.cannon_timer.tick(time.delta());
    }
}

pub fn user_input_system(
    commands: Commands,
    audio: Res<Audio>,
    mut state: ResMut<State<AppState>>,
    mut gamestate: ResMut<State<AppGameState>>,
    runstate: ResMut<RunState>,
    input: Res<Input<KeyCode>>,
    mut physics_time: ResMut<PhysicsTime>,
    mut app_exit_events: EventWriter<AppExit>,
    mut query: Query<(&mut Acceleration, &mut Velocity, &Transform, &mut Ship)>,
) {
    if state.current() != &AppState::StartMenu && input.just_pressed(KeyCode::Back) {
        state.set(AppState::StartMenu).unwrap();
        gamestate.set(AppGameState::Invalid).unwrap();
        physics_time.resume();
    }
    if state.current() == &AppState::Game {
        if gamestate.current() == &AppGameState::Game {
            let player = runstate.player.unwrap();
            let mut rotation = 0;
            let mut thrust = 0;
            if input.pressed(KeyCode::W) {
                thrust += 1
            }
            if input.pressed(KeyCode::A) {
                rotation += 1
            }
            if input.pressed(KeyCode::D) {
                rotation -= 1
            }
            if let Ok((mut acceleration, mut velocity, transform, ship)) = query.get_mut(player) {
                if rotation != 0 {
                    velocity.angular =
                        AxisAngle::new(Vec3::Z, rotation as f32 * ship.rotation_speed);
                }
                acceleration.linear = transform.rotation * (Vec3::Y * thrust as f32 * ship.thrust);
            }
            if input.pressed(KeyCode::Space) {
                if let Ok((_, _, transform, mut ship)) = query.get_mut(player) {
                    if ship.cannon_timer.finished() {
                        spawn_laser(commands, transform, &runstate, audio);
                        ship.cannon_timer.reset();
                    }
                }
            }
            if input.just_pressed(KeyCode::Escape) {
                gamestate.set(AppGameState::Pause).unwrap();
                physics_time.pause();
            }
        } else if gamestate.current() == &AppGameState::Pause {
            if input.just_pressed(KeyCode::Escape) {
                gamestate.set(AppGameState::Game).unwrap();
                physics_time.resume();
            }
        } else if gamestate.current() == &AppGameState::GameOver {
            if input.just_pressed(KeyCode::Return) {
                state.set(AppState::StartMenu).unwrap();
                gamestate.set(AppGameState::Invalid).unwrap();
            }
            if input.just_pressed(KeyCode::Escape) {
                app_exit_events.send(AppExit);
            }
        }
    } else if state.current() == &AppState::StartMenu {
        if input.just_pressed(KeyCode::Return) {
            state.set(AppState::Game).unwrap();
            gamestate.set(AppGameState::Game).unwrap();
        }
        if input.just_pressed(KeyCode::Escape) {
            app_exit_events.send(AppExit);
        }
    }
}
