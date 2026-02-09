use bevy_renet::renet::ClientId;
use wincode::{SchemaRead, SchemaWrite};

#[derive(SchemaRead, SchemaWrite)]
pub enum ServerMessage {
    ClientJoined(ClientId),
    ClientLeft(ClientId),
}

#[repr(u8)]
pub enum ServerChannel {
    ServerMessages,
}

impl From<ServerChannel> for u8 {
    fn from(value: ServerChannel) -> Self {
        match value {
            ServerChannel::ServerMessages => 0,
        }
    }
}
