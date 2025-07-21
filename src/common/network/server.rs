use bevy::prelude::*;
use std::{net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, time::SystemTime};
use bevy_renet::{
    netcode::{ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication, ServerConfig},
    renet::{ConnectionConfig, RenetClient, RenetServer}
};

use crate::common::PROTOCOL_ID;

pub fn create_dedicated_server(port: u16, config: ConnectionConfig) -> (RenetServer, NetcodeServerTransport) {
    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
    let socket = UdpSocket::bind(server_addr).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    let server = RenetServer::new(config);

    (server, transport)
}

pub fn create_client_connection(port: u16, config: ConnectionConfig) -> (RenetClient, NetcodeClientTransport) {
    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
    let socket = UdpSocket::bind(server_addr).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let client = RenetClient::new(config);

    (client, transport)
}
