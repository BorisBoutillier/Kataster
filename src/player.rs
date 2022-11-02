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

// Tag component to update the exhaust particle effect with speed.
#[derive(Component)]
pub struct ExhaustEffect;

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
        sprite: Sprite {
            custom_size: Some(Vec2::new(30., 20.)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -5.0),
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
        .insert(Collider::ball(13.5))
        .insert(ExternalImpulse::default())
        .insert(Velocity::linear(Vec2::ZERO))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert_bundle(InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map,
        });
    let player_entity = player_entity_builder.id();
    runstate.player = Some(player_entity);
}

pub fn player_dampening_system(
    time: Res<Time>,
    runstate: Res<RunState>,
    mut query: Query<&mut Velocity>,
) {
    if let Ok(mut velocity) = query.get_component_mut::<Velocity>(runstate.player.unwrap()) {
        let elapsed = time.delta_seconds();
        velocity.angvel *= 0.1f32.powf(elapsed);
        velocity.linvel *= 0.4f32.powf(elapsed);
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
    mut query: Query<(&mut ExternalImpulse, &mut Velocity, &Transform, &mut Ship)>,
) {
    if gamestate.current() == &AppGameState::Game {
        let player = runstate.player.unwrap();
        if let Ok(action_state) = action_state_query.get(player) {
            let thrust = if action_state.pressed(PlayerAction::Forward) {
                1.0
            } else {
                0.0
            };
            let rotation = if action_state.pressed(PlayerAction::RotateLeft) {
                1
            } else if action_state.pressed(PlayerAction::RotateRight) {
                -1
            } else {
                0
            };
            let fire = action_state.pressed(PlayerAction::Fire);
            if let Ok((mut impulse, mut velocity, transform, mut ship)) = query.get_mut(player) {
                if rotation != 0 {
                    velocity.angvel = rotation as f32 * ship.rotation_speed;
                }
                impulse.impulse =
                    (transform.rotation * (Vec3::Y * thrust * ship.thrust)).truncate();

                if fire && ship.cannon_timer.finished() {
                    spawn_laser(commands, transform, &runstate, audio);
                    ship.cannon_timer.reset();
                }
            }
        }
    }
}
