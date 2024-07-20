use crate::prelude::*;
use bevy_hanabi::prelude::*;

// Plugin that adds particle effects at different point in the game
// All particle effects are handled in a separate plugin to be
// easily disable when targeting WASM
pub struct ParticleEffectsPlugin;

impl Plugin for ParticleEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin).add_systems(
            Update,
            (add_thrust_particles_to_ship, update_thrust_particles),
        );
    }
}

// Add a Particle Effect to every new Ship created
fn add_thrust_particles_to_ship(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    added_ships: Query<Entity, Added<Ship>>,
) {
    for ship_entity in added_ships.iter() {
        // For Ship exhaust, we store a particle effects on the player

        let writer = ExprWriter::new();
        let lifetime = writer.lit(0.1).expr();
        // Gradient for particle color evolution
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(0.5, 0.4, 0.7, 0.8));
        gradient.add_key(0.5, Vec4::new(1.0, 0.8, 0.0, 0.8));
        gradient.add_key(1.0, Vec4::ZERO);
        let init_pos = SetPositionCone3dModifier {
            height: writer.lit(-5.0).expr(),
            base_radius: writer.lit(2.).expr(),
            top_radius: writer.lit(1.).expr(),
            dimension: ShapeDimension::Volume,
        };
        let init_vel = SetVelocitySphereModifier {
            speed: writer.lit(100.0).uniform(writer.lit(400.0)).expr(),
            center: writer.lit(Vec3::new(0.0, 1.0, 0.0)).expr(),
        };
        let effect = effects.add(
            EffectAsset::new(
                vec![16024],
                Spawner::once(10.0.into(), false),
                writer.finish(),
            )
            .with_name("Exhaust")
            .init(init_pos)
            .init(init_vel)
            .init(SetAttributeModifier::new(Attribute::LIFETIME, lifetime))
            .render(ColorOverLifetimeModifier { gradient })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(2.)),
                screen_space_size: true,
            }),
        );
        commands.entity(ship_entity).with_children(|parent| {
            parent.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effect).with_z_layer_2d(Some(10.)),
                    transform: Transform::from_translation(Vec3::new(0.0, -4.0, 0.0)),
                    ..default()
                },
                ExhaustEffect,
            ));
        });
    }
}

// Trigger a new particle spawning whenever the Ship Impulse is non-0
fn update_thrust_particles(
    player: Query<(&ActionState<PlayerAction>, &Children), Changed<ActionState<PlayerAction>>>,
    mut exhaust_effect: Query<&mut EffectSpawner, With<ExhaustEffect>>,
) {
    for (action_state, children) in player.iter() {
        if action_state.pressed(&PlayerAction::Forward) {
            for &child in children.iter() {
                if let Ok(mut spawner) = exhaust_effect.get_mut(child) {
                    spawner.reset();
                }
            }
        }
    }
}
