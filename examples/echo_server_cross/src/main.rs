use base64::Engine;
use futures::executor::block_on;
use std::{
    net::{SocketAddr, UdpSocket},
    thread,
    time::{Duration, Instant, SystemTime},
};

use log::{debug, info};
use renet2::{
    transport::{
        BoxedSocket, NativeSocket, NetcodeServerTransport, ServerSetupConfig, TransportSocket, WebTransportServer, WebTransportServerConfig,
    },
    ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
};
use renetcode2::ServerAuthentication;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();
    for addr in tokio::net::lookup_host("localhost:4433").await.unwrap() {
        info!("native socket address is {}", addr);
    }
    for addr in tokio::net::lookup_host("localhost:4434").await.unwrap() {
        info!("webtransport socket address is {}", addr);
    }
    let future = server("127.0.0.1:4433".parse().unwrap(), "127.0.0.1:4434".parse().unwrap());
    block_on(future);
}

async fn server(public_addr_native: SocketAddr, public_addr_wt: SocketAddr) {
    let native_socket = NativeSocket::new(UdpSocket::bind(public_addr_native).unwrap()).unwrap();

    let wt_socket = {
        let (transport_config, cert_hash) = WebTransportServerConfig::new_selfsigned(public_addr_wt, 10);
        let cert_hash_b64 = base64::engine::general_purpose::STANDARD.encode(cert_hash.hash.as_ref());
        info!("WT SERVER CERT HASH (PASTE ME TO CLIENTS): {:?}", cert_hash_b64);
        WebTransportServer::new(transport_config, tokio::runtime::Handle::try_current().unwrap()).unwrap()
    };

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let server_config = ServerSetupConfig {
        current_time,
        max_clients: 10,
        protocol_id: 0,
        socket_addresses: vec![vec![native_socket.addr().unwrap()], vec![wt_socket.addr().unwrap()]],
        authentication: ServerAuthentication::Unsecure,
    };
    let mut transport = NetcodeServerTransport::new_with_sockets(
        server_config,
        Vec::from([BoxedSocket::new(native_socket), BoxedSocket::new(wt_socket)]),
    )
    .unwrap();

    debug!("transport created");
    let mut server: RenetServer = RenetServer::new(ConnectionConfig::default());

    let mut received_messages = vec![];
    let mut last_updated = Instant::now();

    loop {
        debug!("server tick");
        let now = Instant::now();
        let duration = now - last_updated;
        last_updated = now;

        transport.update(duration, &mut server).unwrap();
        debug!("server update");

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    info!("Client {} connected.", client_id);
                    server.send_message(client_id, DefaultChannel::Unreliable, b"Welcome to the server!".to_vec());
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    info!("Client {} disconnected: {}", client_id, reason);
                }
            }
        }

        for client_id in server.clients_id() {
            while let Some(message) = server.receive_message(client_id, DefaultChannel::Unreliable) {
                let message = String::from_utf8(message.into()).unwrap();
                info!("Client {} sent text: {}", client_id, message);
                let text = format!("{}: {}", client_id, message);
                received_messages.push(text);
            }
        }

        for text in received_messages.drain(..) {
            server.broadcast_message(DefaultChannel::Unreliable, text.as_bytes().to_vec());
        }

        transport.send_packets(&mut server);
        thread::sleep(Duration::from_millis(50));
    }
}
