use bevy::prelude::*;
use bevy_rapier2d::{
    physics::RigidBodyHandleComponent, rapier::dynamics::JointSet,
    rapier::dynamics::RigidBodyHandle, rapier::dynamics::RigidBodySet,
    rapier::geometry::BroadPhase, rapier::geometry::ColliderSet, rapier::geometry::NarrowPhase,
    rapier::pipeline::PhysicsPipeline,
};
use std::collections::HashMap;

/// Plugin for additional Rapier2d features.
/// . A BodyHandleToEntity and EntityToBodyHandle resource.
/// . An automatic rigid_body removal from rapier2d, requiring the existence of CLEANUP stage
///   This stage must be after all stages that can despawn entities containing a RigidBody
pub struct RapierUtilsPlugin;

impl Plugin for RapierUtilsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(BodyHandleToEntity(HashMap::new()))
            .add_resource(EntityToBodyHandle(HashMap::new()))
            .add_system(body_to_entity_system.system())
            .add_system_to_stage("CLEANUP", remove_rigid_body_system.system());
    }
}
pub struct BodyHandleToEntity(pub HashMap<RigidBodyHandle, Entity>);
pub struct EntityToBodyHandle(pub HashMap<Entity, RigidBodyHandle>);

/// Keeps BodyHandleToEntity, EntityToBodyHandle resource in sync.
fn body_to_entity_system(
    mut bh_to_e: ResMut<BodyHandleToEntity>,
    mut e_to_bh: ResMut<EntityToBodyHandle>,
    mut added: Query<(Entity, Added<RigidBodyHandleComponent>)>,
) {
    for (entity, body_handle) in &mut added.iter() {
        bh_to_e.0.insert(body_handle.handle(), entity);
        e_to_bh.0.insert(entity, body_handle.handle());
    }
}

/// Detects when a RigidBodyHandle is removed from an entity, as it despawns
/// And inform rapier about the removal
/// Requires the EntityToBodyHandle resource, as the
fn remove_rigid_body_system(
    mut pipeline: ResMut<PhysicsPipeline>,
    mut broad_phase: ResMut<BroadPhase>,
    mut narrow_phase: ResMut<NarrowPhase>,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut joints: ResMut<JointSet>,
    mut e_to_bh: ResMut<EntityToBodyHandle>,
    mut bh_to_e: ResMut<BodyHandleToEntity>,
    query: Query<&RigidBodyHandleComponent>,
) {
    for entity in query.removed::<RigidBodyHandleComponent>().iter() {
        let handle = e_to_bh.0.get(entity).unwrap();
        pipeline.remove_rigid_body(
            *handle,
            &mut broad_phase,
            &mut narrow_phase,
            &mut bodies,
            &mut colliders,
            &mut joints,
        );
        bh_to_e.0.remove(handle);
        e_to_bh.0.remove(entity);
    }
}
