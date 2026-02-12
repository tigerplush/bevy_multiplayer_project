use bevy::prelude::*;
use bevy_renet::RenetServer;
use server::*;

use crate::server_plugin::*;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_observer(on_client_connect)
            .add_observer(on_client_disconnect);
    }
}

fn on_client_connect(
    add: On<Add, Client>,
    active_clients: Res<ActiveClients>,
    mut server: ResMut<RenetServer>,
    query: Query<&Client>,
) {
    let client = query.get(add.entity).unwrap();
    let message = wincode::serialize(&ServerMessage::ClientJoined(client.0)).unwrap();
    for (client_id, _) in active_clients.iter() {
        server.send_message(*client_id, ServerChannel::ServerMessages, message.clone());
    }
}

fn on_client_disconnect(
    remove: On<Remove, Client>,
    active_clients: Res<ActiveClients>,
    mut server: ResMut<RenetServer>,
    query: Query<&Client>,
) {
    let client = query.get(remove.entity).unwrap();
    let message = wincode::serialize(&ServerMessage::ClientLeft(client.0)).unwrap();
    for (client_id, _) in active_clients.iter() {
        server.send_message(*client_id, ServerChannel::ServerMessages, message.clone());
    }
}