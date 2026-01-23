use std::{net::UdpSocket, time::SystemTime};

use bevy::{log::LogPlugin, prelude::*};
use bevy_renet::{
    RenetServer, RenetServerEvent, RenetServerPlugin, netcode::{NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerConfig}, renet::ConnectionConfig
};

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
            .insert_resource(server)
            .insert_resource(transport)
            .add_observer(handle_server_events);
    }
}

fn handle_server_events(server_event: On<RenetServerEvent>) {
    info!("{:?}", server_event);
}
