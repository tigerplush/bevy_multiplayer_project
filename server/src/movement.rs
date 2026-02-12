use bevy::prelude::*;
use server::PlayerMovementIntention;

#[derive(Component, Default)]
pub(crate) struct Velocity(pub Vec3);

pub(crate) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_velocity, update_transform).chain());
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
