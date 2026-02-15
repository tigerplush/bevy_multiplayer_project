use std::collections::HashMap;

use bevy::prelude::*;
use bevy_renet::renet::ClientId;
use wincode::{SchemaRead, SchemaWrite};

/// Describes which messages the server can send via the ServerMessages channel.
#[derive(SchemaRead, SchemaWrite)]
pub enum ServerMessage {
    ClientJoined(ClientId),
    ClientLeft(ClientId),
}

/// Describes which channels a server can send messages on.
#[repr(u8)]
pub enum ServerChannel {
    ServerMessages,
    NetworkedEntities,
}

impl From<ServerChannel> for u8 {
    fn from(value: ServerChannel) -> Self {
        match value {
            ServerChannel::ServerMessages => 0,
            ServerChannel::NetworkedEntities => 1,
        }
    }
}

/// Describes which channel a client can send messages from.
#[repr(u8)]
pub enum ClientChannel {
    ClientInput,
}

impl From<ClientChannel> for u8 {
    fn from(value: ClientChannel) -> Self {
        match value {
            ClientChannel::ClientInput => 0,
        }
    }
}

///
#[derive(Component, Debug, Resource, SchemaRead, SchemaWrite)]
pub struct PlayerMovementIntention {
    pub translation: [f32; 2],
    pub rotation: [f32; 2],
}

impl PlayerMovementIntention {
    pub const fn new() -> Self {
        PlayerMovementIntention {
            translation: [0.0; 2],
            rotation: [0.0; 2],
        }
    }
}

#[derive(Debug, Default, Deref, DerefMut, SchemaRead, SchemaWrite)]
pub struct NetworkedEntities(pub Vec<TransformUpdate>);

#[derive(Debug, SchemaRead, SchemaWrite)]
pub struct TransformUpdate {
    pub client_id: ClientId,
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
}

impl TransformUpdate {
    pub fn new(
        client_id: ClientId,
        translation: impl Into<[f32; 3]>,
        rotation: impl Into<[f32; 4]>,
    ) -> Self {
        TransformUpdate {
            client_id,
            translation: translation.into(),
            rotation: rotation.into(),
        }
    }
}

#[derive(Component, Resource)]
pub struct Client(pub ClientId);

#[derive(Deref, DerefMut, Resource)]
pub struct ActiveClients(HashMap<ClientId, Entity>);

impl ActiveClients {
    pub fn empty() -> Self {
        ActiveClients(HashMap::new())
    }
}
