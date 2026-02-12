use bevy::prelude::*;
use bevy_renet::RenetClient;
use client::{ClientChannel, PlayerMovementIntention};
use leafwing_input_manager::prelude::*;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerActions>::default())
            .insert_resource(PlayerMovementIntention::new())
            .add_systems(Startup, setup)
            .add_systems(Update, move_player)
            .add_systems(PostUpdate, send_movement);
    }
}

#[derive(Actionlike, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
enum PlayerActions {
    #[actionlike(DualAxis)]
    Move,
}

fn setup(mut commands: Commands) {
    let input_map = InputMap::default().with_dual_axis(PlayerActions::Move, VirtualDPad::wasd());
    commands.spawn(input_map);
}

fn move_player(
    actions: Single<&ActionState<PlayerActions>>,
    mut player_movement: ResMut<PlayerMovementIntention>,
) {
    let input = actions.clamped_axis_pair(&PlayerActions::Move);
    player_movement.x = input.x;
    player_movement.y = input.y;
}

fn send_movement(player_movement: Res<PlayerMovementIntention>, mut client: ResMut<RenetClient>) {
    let message = wincode::serialize(&*player_movement).unwrap();
    client.send_message(ClientChannel::ClientInput, message);
}
