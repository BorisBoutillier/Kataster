use bevy::prelude::*;
use bevy_rapier2d::{
    na::Vector2, physics::EntityToBody, physics::EventQueue, physics::Gravity,
    physics::RapierPhysicsScale, physics::RigidBodyHandleComponent,
    rapier::dynamics::IntegrationParameters, rapier::dynamics::JointSet,
    rapier::dynamics::RigidBodyHandle, rapier::dynamics::RigidBodySet,
    rapier::geometry::BroadPhase, rapier::geometry::ColliderSet, rapier::geometry::NarrowPhase,
    rapier::pipeline::PhysicsPipeline,
};
/// This is a copy of the bevy_rapier RapierPhysicsPlugin, with :
/// . a custom version of the step_world_system, supporting to be paused
/// . a BodyHandleToEntity resources
/// . 0 Gravity
use std::collections::HashMap;
pub struct BodyHandleToEntity(pub HashMap<RigidBodyHandle, Entity>);

/// Keeps BodyHandleToEntity resource in sync.
// TODO: handle removals.
pub fn body_to_entity_system(
    mut h_to_e: ResMut<BodyHandleToEntity>,
    mut added: Query<(Entity, Added<RigidBodyHandleComponent>)>,
) {
    for (entity, body_handle) in &mut added.iter() {
        h_to_e.0.insert(body_handle.handle(), entity);
    }
}

pub struct RapierPipelineActive(pub bool);

/// System responsible for performing one timestep of the physics world, if not paused
pub fn my_step_world_system(
    gravity: Res<Gravity>,
    integration_parameters: Res<IntegrationParameters>,
    active: Res<RapierPipelineActive>,
    mut pipeline: ResMut<PhysicsPipeline>,
    mut broad_phase: ResMut<BroadPhase>,
    mut narrow_phase: ResMut<NarrowPhase>,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut joints: ResMut<JointSet>,
    events: Res<EventQueue>,
) {
    if events.auto_clear {
        events.clear();
    }

    if active.0 {
        pipeline.step(
            &gravity.0,
            &integration_parameters,
            &mut broad_phase,
            &mut narrow_phase,
            &mut bodies,
            &mut colliders,
            &mut joints,
            &*events,
        );
    }
}

/// A copy of the bevy_rapier RapierPhysicsPlugin, with a custom the step_world_system, handling Pause
pub struct MyRapierPhysicsPlugin;

impl Plugin for MyRapierPhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(PhysicsPipeline::new())
            .add_resource(IntegrationParameters::default())
            .add_resource(Gravity(Vector2::zeros()))
            .add_resource(BroadPhase::new())
            .add_resource(NarrowPhase::new())
            .add_resource(RigidBodySet::new())
            .add_resource(ColliderSet::new())
            .add_resource(JointSet::new())
            .add_resource(RapierPhysicsScale(1.0))
            .add_resource(EventQueue::new(true))
            // TODO: can we avoid this map? We are only using this
            // to avoid some borrowing issue when joints creations
            // are needed.
            .add_resource(EntityToBody::new())
            // Custom resources of my plugin
            .add_resource(BodyHandleToEntity(HashMap::new()))
            .add_resource(RapierPipelineActive(true))
            .add_system_to_stage_front(
                stage::PRE_UPDATE,
                bevy_rapier2d::physics::create_body_and_collider_system.system(),
            )
            .add_system_to_stage(
                stage::PRE_UPDATE,
                bevy_rapier2d::physics::create_joints_system.system(),
            )
            // Custom version of the step_world_system
            .add_system_to_stage(stage::UPDATE, my_step_world_system.system())
            // Additional system to keep BodyHandleToEntity up to date
            .add_system(body_to_entity_system.system())
            .add_system_to_stage(
                stage::POST_UPDATE,
                bevy_rapier2d::physics::sync_transform_system.system(),
            );
    }
}
