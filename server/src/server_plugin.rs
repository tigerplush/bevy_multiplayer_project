use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy::{log::LogPlugin, prelude::*};
use bevy_renet::{
    RenetServer, RenetServerEvent, RenetServerPlugin,
    netcode::{NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerConfig},
    renet::{ClientId, ConnectionConfig, ServerEvent},
};
use server::{ClientChannel, PlayerMovementIntention, ServerChannel, ServerMessage};

use crate::player::PlayerPlugin;

pub(crate) struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let server = RenetServer::new(ConnectionConfig::default());
        let server_address = "127.0.0.1:5000".parse().unwrap();
        let socket = UdpSocket::bind(server_address).unwrap();
        let server_config = ServerConfig {
            current_time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap(),
            max_clients: 64,
            protocol_id: 0,
            public_addresses: vec![server_address],
            authentication: ServerAuthentication::Unsecure,
        };
        let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
        app.add_plugins(MinimalPlugins)
            .add_plugins(LogPlugin::default())
            .add_plugins(RenetServerPlugin)
            .add_plugins(NetcodeServerPlugin)
            .add_plugins(PlayerPlugin)
            .insert_resource(ActiveClients::empty())
            .insert_resource(server)
            .insert_resource(transport)
            .add_systems(Update, handle_client_messages)
            .add_observer(handle_server_events);
    }
}

#[derive(Component)]
pub(crate) struct Client(pub ClientId);

#[derive(Deref, DerefMut, Resource)]
pub(crate) struct ActiveClients(HashMap<ClientId, Entity>);

impl ActiveClients {
    fn empty() -> Self {
        ActiveClients(HashMap::new())
    }
}

fn handle_server_events(
    server_event: On<RenetServerEvent>,
    mut active_clients: ResMut<ActiveClients>,
    mut server: ResMut<RenetServer>,
    mut commands: Commands,
) {
    match **server_event {
        // On client, spawn representation
        ServerEvent::ClientConnected { client_id } => {
            info!("Client {} connected", client_id);
            let entity = commands.spawn(Client(client_id)).id();
            // inform the new client about all already connected clients
            for (already_connected_id, _) in active_clients.iter() {
                let message =
                    wincode::serialize(&ServerMessage::ClientJoined(*already_connected_id))
                        .unwrap();
                server.send_message(client_id, ServerChannel::ServerMessages, message);
            }
            active_clients.insert(client_id, entity);
        }
        // On disconnect, despawn representation
        ServerEvent::ClientDisconnected { client_id, reason } => {
            info!("Client {} disconnected, reason: {}", client_id, reason);
            if let Some(entity) = active_clients.remove(&client_id) {
                commands.entity(entity).despawn();
            }
        }
    }
}


fn handle_client_messages(active_clients: Res<ActiveClients>, mut server: ResMut<RenetServer>) {
    for (client_id, _) in active_clients.iter() {
        while let Some(message) = server.receive_message(*client_id, ClientChannel::ClientInput) {
            let movement = wincode::deserialize::<PlayerMovementIntention>(&message).unwrap();
        }
    }
}
