use crate::prelude::*;
use bevy_hanabi::prelude::*;

// Plugin that adds particle effects at different point in the game
// All particle effects are handled in a separate plugin to be
// easily disable when targeting WASM
pub struct ParticleEffectsPlugin;

impl Plugin for ParticleEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HanabiPlugin)
            .add_system(add_thrust_particles_to_ship)
            .add_system(update_thrust_particles);
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
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(0.5, 0.4, 0.7, 0.8));
        gradient.add_key(0.5, Vec4::new(1.0, 0.8, 0.0, 0.8));
        gradient.add_key(1.0, Vec4::ZERO);
        let effect = effects.add(
            EffectAsset {
                name: "Exhaust".to_string(),
                capacity: 16024,
                spawner: Spawner::once(10.0.into(), false),
                //spawner: Spawner::rate(500.0.into()),
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
        commands.entity(ship_entity).add_children(|parent| {
            parent
                .spawn_bundle(ParticleEffectBundle {
                    effect: ParticleEffect::new(effect),
                    transform: Transform::from_translation(Vec3::new(0.0, -30.0, 0.0)),
                    ..Default::default()
                })
                .insert(ExhaustEffect);
        });
    }
}

// Trigger a new particle spawning whenever the Ship Impulse is non-0
fn update_thrust_particles(
    impulse: Query<(&ExternalImpulse, &Children), Changed<ExternalImpulse>>,
    mut exhaust_effet: Query<&mut ParticleEffect, With<ExhaustEffect>>,
) {
    for (impulse, children) in impulse.iter() {
        if impulse.impulse.length() != 0.0 {
            for &child in children.iter() {
                if let Ok(mut effect) = exhaust_effet.get_mut(child) {
                    if let Some(spawner) = effect.maybe_spawner() {
                        spawner.reset();
                    }
                }
            }
        }
    }
}
