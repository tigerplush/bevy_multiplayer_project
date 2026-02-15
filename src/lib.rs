use bevy::prelude::*;
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

#[derive(Component, Debug, Resource, SchemaRead, SchemaWrite)]
pub struct PlayerMovementIntention {
    pub x: f32,
    pub y: f32,
}

impl PlayerMovementIntention {
    pub const fn new() -> Self {
        PlayerMovementIntention { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug, Default, SchemaRead, SchemaWrite)]
pub struct NetworkedEntities {
    pub clients: Vec<ClientId>,
    pub translation: Vec<[f32;3]>,
}