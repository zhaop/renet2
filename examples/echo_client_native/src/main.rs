use std::{
    net::{SocketAddr, UdpSocket},
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
    time::{Duration, Instant, SystemTime},
};

use renet2::{
    transport::{ClientAuthentication, NativeSocket, NetcodeClientTransport},
    ConnectionConfig, DefaultChannel, RenetClient,
};

fn main() {
    env_logger::init();
    println!("Type to enter a message.");

    let server_addr: SocketAddr = "127.0.0.1:4433".parse().unwrap();
    let client_socket = NativeSocket::new(UdpSocket::bind("127.0.0.1:0").unwrap()).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        socket_id: 0, // Socket id 0 for native sockets in this example.
        server_addr,
        client_id,
        user_data: None,
        protocol_id: 0,
    };

    let mut transport = NetcodeClientTransport::new(current_time, authentication, client_socket).unwrap();

    let connection_config = ConnectionConfig::default();
    let mut client = RenetClient::new(connection_config);

    let stdin_channel: Receiver<String> = spawn_stdin_channel();
    let mut last_updated = Instant::now();
    loop {
        let now = Instant::now();
        let duration = now - last_updated;
        last_updated = now;

        client.update(duration);
        transport.update(duration, &mut client).unwrap();

        if client.is_connected() {
            match stdin_channel.try_recv() {
                Ok(text) => client.send_message(DefaultChannel::Unreliable, text.as_bytes().to_vec()),
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
            }

            while let Some(text) = client.receive_message(DefaultChannel::Unreliable) {
                let text = String::from_utf8(text.into()).unwrap();
                println!("{}", text);
            }
        }

        transport.send_packets(&mut client).unwrap();
        thread::sleep(Duration::from_millis(50));
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer.trim_end().to_string()).unwrap();
    });
    rx
}
