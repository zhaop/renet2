use std::{io::ErrorKind, net::SocketAddr};

use crate::transport::{NetcodeTransportError, TransportSocket};

use super::*;

/// Implementation of [`TransportSocket`] for a server socket using in-memory channels.
///
/// In-memory sockets are treated as encrypted by default for efficiency. Use [`Self::new_with`] to use
/// a different policy (this is useful for performance tests).
#[derive(Debug)]
pub struct MemorySocketServer {
    clients: Vec<MemorySocketChannels>,
    encrypted: bool,
    drain_index: usize,
}

impl MemorySocketServer {
    /// Makes a new in-memory socket for a server.
    pub fn new(clients: Vec<MemorySocketChannels>) -> Self {
        Self::new_with(clients, true)
    }

    /// Makes a new in-memory socket for a server with a specific encryption policy.
    pub fn new_with(clients: Vec<MemorySocketChannels>, encrypted: bool) -> Self {
        assert!(clients.len() < u16::MAX as usize);
        Self {
            clients,
            encrypted,
            drain_index: 0,
        }
    }
}

impl TransportSocket for MemorySocketServer {
    fn is_encrypted(&self) -> bool {
        self.encrypted
    }

    fn addr(&self) -> std::io::Result<SocketAddr> {
        Ok(in_memory_server_addr())
    }

    fn is_closed(&mut self) -> bool {
        false
    }

    fn close(&mut self) {}
    fn disconnect(&mut self, _: SocketAddr) {}
    fn preupdate(&mut self) {
        self.drain_index = 0;
    }

    fn try_recv(&mut self, buffer: &mut [u8]) -> std::io::Result<(usize, SocketAddr)> {
        loop {
            if self.drain_index >= self.clients.len() {
                return Err(std::io::Error::from(ErrorKind::WouldBlock));
            }

            if let Ok(packet) = self.clients[self.drain_index].receiver.try_recv() {
                buffer[..packet.len].copy_from_slice(&packet.bytes[..packet.len]);
                return Ok((packet.len, in_memory_client_addr(self.drain_index as u16)));
            };

            self.drain_index += 1;
        }
    }

    fn postupdate(&mut self) {}

    fn send(&mut self, addr: SocketAddr, packet: &[u8]) -> Result<(), NetcodeTransportError> {
        assert!(packet.len() <= NETCODE_MAX_PACKET_BYTES);

        let client_id = addr.port() as usize;
        assert!(client_id < self.clients.len());

        let mut mem_packet = InMemoryPacket {
            len: packet.len(),
            ..Default::default()
        };
        mem_packet.bytes[..packet.len()].copy_from_slice(packet);
        self.clients[client_id]
            .sender
            .send(mem_packet)
            .map_err(|_| std::io::Error::from(ErrorKind::ConnectionAborted))?;

        Ok(())
    }
}
