use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy::{log::LogPlugin, prelude::*};
use bevy_renet::{
    RenetServer, RenetServerEvent, RenetServerPlugin,
    netcode::{NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerConfig},
    renet::{ClientId, ConnectionConfig, ServerEvent},
};
use server::{ServerChannel, ServerMessage};

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
            .insert_resource(ActiveClients::empty())
            .insert_resource(server)
            .insert_resource(transport)
            .add_observer(handle_server_events)
            .add_observer(on_client_connect)
            .add_observer(on_client_disconnect);
    }
}

#[derive(Component)]
struct Client(ClientId);

#[derive(Deref, DerefMut, Resource)]
struct ActiveClients(HashMap<ClientId, Entity>);

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
