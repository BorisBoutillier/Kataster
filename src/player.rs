use crate::prelude::*;

pub const START_LIFE: u32 = 3;

// Actions are divided in two enums
// One for pure Player Ship actions, during effective gameplay, added on the player entity itself.
// One for Menu actions, added as a global resource

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    Forward,
    RotateLeft,
    RotateRight,
    Fire,
}

pub fn spawn_player(
    mut commands: Commands,
    mut runstate: ResMut<RunState>,
    asset_server: Res<AssetServer>,
) {
    // For player actions, allow both keyboard WASD and Arrows to control the ship
    let input_map = InputMap::new([
        (KeyCode::W, PlayerAction::Forward),
        (KeyCode::Up, PlayerAction::Forward),
        (KeyCode::A, PlayerAction::RotateLeft),
        (KeyCode::Left, PlayerAction::RotateLeft),
        (KeyCode::D, PlayerAction::RotateRight),
        (KeyCode::Right, PlayerAction::RotateRight),
        (KeyCode::Space, PlayerAction::Fire),
    ]);
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
        )
        .insert_bundle(InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map,
        });
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

pub fn ship_input_system(
    commands: Commands,
    audio: Res<Audio>,
    gamestate: Res<State<AppGameState>>,
    runstate: ResMut<RunState>,
    action_state_query: Query<&ActionState<PlayerAction>>,
    mut query: Query<(&mut Acceleration, &mut Velocity, &Transform, &mut Ship)>,
) {
    if gamestate.current() == &AppGameState::Game {
        let player = runstate.player.unwrap();
        if let Ok(action_state) = action_state_query.get(player) {
            let thrust = if action_state.pressed(PlayerAction::Forward) {
                1
            } else {
                0
            };
            let rotation = if action_state.pressed(PlayerAction::RotateLeft) {
                1
            } else if action_state.pressed(PlayerAction::RotateRight) {
                -1
            } else {
                0
            };
            let fire = action_state.pressed(PlayerAction::Fire);
            if let Ok((mut acceleration, mut velocity, transform, mut ship)) = query.get_mut(player)
            {
                if rotation != 0 {
                    velocity.angular =
                        AxisAngle::new(Vec3::Z, rotation as f32 * ship.rotation_speed);
                }
                acceleration.linear = transform.rotation * (Vec3::Y * thrust as f32 * ship.thrust);
                if fire && ship.cannon_timer.finished() {
                    spawn_laser(commands, transform, &runstate, audio);
                    ship.cannon_timer.reset();
                }
            }
        }
    }
}
