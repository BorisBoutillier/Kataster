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
    mut effects: ResMut<Assets<EffectAsset>>,
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
    // For Ship exhaust, we store a particle effects on the player
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.5, 0.4, 0.7, 0.8));
    gradient.add_key(0.5, Vec4::new(1.0, 0.8, 0.0, 0.8));
    gradient.add_key(1.0, Vec4::ZERO);
    let effect = effects.add(
        EffectAsset {
            name: "Exhaust".to_string(),
            capacity: 16024,
            spawner: Spawner::once(10.0.into(), false),
            z_layer_2d: 10.0,
            ..Default::default()
        }
        .init(ParticleLifetimeModifier { lifetime: 0.1 })
        .init(PositionCone3dModifier {
            height: -5.0,
            base_radius: 5.,
            top_radius: 3.0,
            speed: Value::Uniform((100.0, 400.0)),
            dimension: ShapeDimension::Volume,
        })
        .render(ColorOverLifetimeModifier { gradient })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant(Vec2::splat(0.2)),
        }),
    );
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
            thrust: 6.0,
            life: START_LIFE,
            cannon_timer: Timer::from_seconds(0.2, false),
        })
        .insert(ForState {
            states: vec![AppState::Game],
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(5.0 * 10.0))
        .insert(ExternalImpulse::default())
        .insert(Velocity::linear(Vec2::ZERO))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert_bundle(InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map,
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ParticleEffectBundle {
                    effect: ParticleEffect::new(effect),
                    transform: Transform::from_translation(Vec3::new(0.0, -30.0, 0.0)),
                    ..Default::default()
                })
                .insert(ExhaustEffect);
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
    mut exhaust_effet: Query<&mut ParticleEffect, With<ExhaustEffect>>,
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

                // Spawn a set of particle if we have thrust to simulate the exhaust
                if thrust != 0. {
                    if let Ok(mut effect) = exhaust_effet.get_single_mut() {
                        if let Some(spawner) = effect.maybe_spawner() {
                            spawner.reset();
                        }
                    }
                }
                if fire && ship.cannon_timer.finished() {
                    spawn_laser(commands, transform, &runstate, audio);
                    ship.cannon_timer.reset();
                }
            }
        }
    }
}
