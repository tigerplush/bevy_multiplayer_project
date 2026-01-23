use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{
    RenetClient, RenetClientPlugin,
    netcode::{ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport, NetcodeErrorEvent},
    renet::ConnectionConfig,
};

pub(crate) struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        let client = RenetClient::new(ConnectionConfig::default());
        let server_address = "127.0.0.1:5000".parse().unwrap();
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let authentication = ClientAuthentication::Unsecure {
            protocol_id: 0,
            client_id: 0,
            server_addr: server_address,
            user_data: None,
        };
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
        app.add_plugins(DefaultPlugins)
            .add_plugins(RenetClientPlugin)
            .add_plugins(NetcodeClientPlugin)
            .insert_resource(client)
            .insert_resource(transport)
            .add_observer(on_error);
    }
}

fn on_error(error: On<NetcodeErrorEvent>) {
    error!("{:?}", error);
}
