use bevy::prelude::*;
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodyHandle};
use std::collections::HashMap;
pub struct BodyHandleToEntity(pub HashMap<RigidBodyHandle, Entity>);

pub fn body_to_entity_system(
    mut h_to_e: ResMut<BodyHandleToEntity>,
    mut added: Query<(Entity, Added<RigidBodyHandleComponent>)>,
) {
    for (entity, body_handle) in &mut added.iter() {
        h_to_e.0.insert(body_handle.handle(), entity);
    }
}
