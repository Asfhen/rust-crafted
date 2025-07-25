use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        resource::Resource
    },
    math::Vec3,
    platform::collections::HashMap,
};
use bevy_renet::renet::{ChannelConfig, ClientId, SendType};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const PROTOCOL_ID: u64 = 1234;

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<ClientId, Entity>,
}

#[derive(Component)]
pub struct NetworkConfig {
    pub server_config: Vec<ChannelConfig>,
    pub client_config: Vec<ChannelConfig>,
    pub bytes_per_tick: u64,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            server_config: vec![
                ChannelConfig {
                    channel_id: 0,
                    max_memory_usage_bytes: 1024 * 1024 * 5,
                    send_type: SendType::ReliableOrdered {
                        resend_time: Duration::from_millis(300),
                    },
                },
                ChannelConfig {
                    channel_id: 1,
                    max_memory_usage_bytes: 1024 * 1024 * 2,
                    send_type: SendType::Unreliable,
                },
            ],
            client_config: vec![ChannelConfig {
                channel_id: 0,
                max_memory_usage_bytes: 1024 * 1024 * 5,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(300),
                },
            }],
            bytes_per_tick: 1024 * 1024 * 7,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkMessage {
    PlayerConnected { id: ClientId },
    PlayerDisconnected { id: ClientId },
    ChunkData(Vec<u8>),
    PlayerPosition(Vec3),
}
