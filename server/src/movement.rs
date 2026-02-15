use bevy::prelude::*;
use bevy_renet::RenetServer;
use server::{Client, NetworkedEntities, PlayerMovementIntention, ServerChannel, TransformUpdate};

#[derive(Component, Default)]
pub(crate) struct Velocity(pub Vec3);

pub(crate) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_rotation, update_velocity, update_transform, sync_networked_entities).chain(),
        );
    }
}

fn update_rotation(time: Res<Time>, mut query: Query<(&PlayerMovementIntention, &mut Transform)>) {
    for (movement_intent, mut transform) in &mut query {
        let yaw = Quat::from_rotation_y(movement_intent.rotation[0] * time.delta_secs());
        transform.rotation = yaw * transform.rotation;
    }
}

fn update_velocity(mut query: Query<(&PlayerMovementIntention, &mut Velocity)>) {
    for (movement_intent, mut velocity) in &mut query {
        velocity.0 = Vec3::new(movement_intent.translation[0], 0.0, movement_intent.translation[1]);
    }
}

fn update_transform(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in &mut query {
        let forward = transform.forward();
        let right = transform.right();

        let movement = forward * velocity.0.z + right * velocity.0.x;

        transform.translation += movement * time.delta_secs();
    }
}

fn sync_networked_entities(mut server: ResMut<RenetServer>, query: Query<(&Client, &Transform)>) {
    let mut networked_entities = NetworkedEntities::default();
    for (client, transform) in &query {
        networked_entities.push(TransformUpdate::new(client.0, transform.translation, transform.rotation));
    }
    let message = wincode::serialize(&networked_entities).unwrap();
    server.broadcast_message(ServerChannel::NetworkedEntities, message);
}
