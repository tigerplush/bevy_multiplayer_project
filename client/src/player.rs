use bevy::prelude::*;
use bevy_renet::RenetClient;
use client::{Client, ClientChannel, PlayerMovementIntention};
use leafwing_input_manager::prelude::*;

use crate::client_plugin::LocalPlayer;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerActions>::default())
            .insert_resource(PlayerMovementIntention::new())
            .add_systems(Startup, setup)
            .add_systems(Update, move_player)
            .add_systems(PostUpdate, send_movement)
            .add_observer(on_client_connect)
            .add_observer(on_local_player);
    }
}

#[derive(Actionlike, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
enum PlayerActions {
    #[actionlike(DualAxis)]
    Move,
    #[actionlike(DualAxis)]
    LookAround,
}

fn setup(mut commands: Commands) {
    let input_map = InputMap::default()
        .with_dual_axis(PlayerActions::Move, VirtualDPad::wasd())
        .with_dual_axis(PlayerActions::LookAround, MouseMove::default().inverted_x());
    commands.spawn(input_map);
}

fn move_player(
    actions: Single<&ActionState<PlayerActions>>,
    mut player_movement: ResMut<PlayerMovementIntention>,
) {
    let translation = actions.clamped_axis_pair(&PlayerActions::Move);
    let rotation = actions.clamped_axis_pair(&PlayerActions::LookAround);
    player_movement.translation = translation.into();
    player_movement.rotation = rotation.into();
}

fn send_movement(player_movement: Res<PlayerMovementIntention>, mut client: ResMut<RenetClient>) {
    let message = wincode::serialize(&*player_movement).unwrap();
    client.send_message(ClientChannel::ClientInput, message);
}

fn on_client_connect(
    add: On<Add, Client>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands.entity(add.entity).insert((
        Transform::default(),
        Visibility::Inherited,
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
    ));
}

fn on_local_player(add: On<Add, LocalPlayer>, mut commands: Commands) {
    commands.entity(add.entity).insert(children![(
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.0, 0.0)
    )]);
}
