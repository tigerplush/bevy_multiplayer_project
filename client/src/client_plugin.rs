use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{
    RenetClient, RenetClientPlugin,
    netcode::{
        ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport, NetcodeErrorEvent,
    },
    renet::ConnectionConfig,
};
use client::{ActiveClients, Client, NetworkedEntities, ServerChannel, ServerMessage};

use crate::player::PlayerPlugin;

pub(crate) struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        let client = RenetClient::new(ConnectionConfig::default());
        let server_address = "127.0.0.1:5000".parse().unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let client_id = current_time.as_millis() as u64;
        let authentication = ClientAuthentication::Unsecure {
            protocol_id: 0,
            client_id,
            server_addr: server_address,
            user_data: None,
        };
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
        app.add_plugins(DefaultPlugins)
            .add_plugins(RenetClientPlugin)
            .add_plugins(NetcodeClientPlugin)
            .add_plugins(PlayerPlugin)
            .insert_resource(client)
            .insert_resource(transport)
            .insert_resource(ActiveClients::empty())
            .add_observer(on_error)
            .add_systems(Startup, setup)
            .add_systems(Update, (handle_server_messages, sync_networked_entities));
    }
}

fn on_error(error: On<NetcodeErrorEvent>) {
    error!("{:?}", error);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn handle_server_messages(
    mut client: ResMut<RenetClient>,
    mut active_clients: ResMut<ActiveClients>,
    mut commands: Commands,
) {
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = wincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessage::ClientJoined(client_id) => {
                info!("Client {} joined", client_id);
                let entity = commands.spawn(Client(client_id)).id();
                active_clients.insert(client_id, entity);
            }
            ServerMessage::ClientLeft(client_id) => {
                info!("Client {} left", client_id);
                if let Some(entity) = active_clients.remove(&client_id) {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn sync_networked_entities(
    mut client: ResMut<RenetClient>,
    active_clients: Res<ActiveClients>,
    mut query: Query<&mut Transform>,
) {
    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        let networked_entities = wincode::deserialize::<NetworkedEntities>(&message).unwrap();
        for (index, client_id) in networked_entities.clients.iter().enumerate() {
            if let Some(entity) = active_clients.get(client_id) {
                if let Ok(mut transform) = query.get_mut(*entity) {
                    transform.translation = networked_entities.translation[index].into();
                }
            }
        }
    }
}
