use bevy::prelude::*;
use bevy_renet::RenetServer;
use server::{Client, NetworkedEntities, PlayerMovementIntention, ServerChannel};

#[derive(Component, Default)]
pub(crate) struct Velocity(pub Vec3);

pub(crate) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_velocity, update_transform, sync_networked_entities).chain(),
        );
    }
}

fn update_velocity(mut query: Query<(&PlayerMovementIntention, &mut Velocity)>) {
    for (movement_intent, mut velocity) in &mut query {
        velocity.0 = Vec3::new(movement_intent.x, 0.0, movement_intent.y);
    }
}

fn update_transform(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in &mut query {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

fn sync_networked_entities(mut server: ResMut<RenetServer>, query: Query<(&Client, &Transform)>) {
    let mut networked_entities = NetworkedEntities::default();
    for (client, transform) in &query {
        networked_entities.clients.push(client.0);
        networked_entities.translation.push(transform.translation.into());
    }
    let message = wincode::serialize(&networked_entities).unwrap();
    server.broadcast_message(ServerChannel::NetworkedEntities, message);
}
