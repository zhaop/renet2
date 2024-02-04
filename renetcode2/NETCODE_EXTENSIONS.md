# Netcode Extensions (version 'a')

`renet2` extends the original [netcode](https://github.com/networkprotocol/netcode) protocol.


## Optional Encryption

`Netcode` was designed for UDP, where packets are unencrypted. Alternate data transports such as WebTransport provide their own encryption. To avoid the performance cost of double-encryption, we extend `netcode` so encryption is optional based on the underlying transport.

### 1. Unencrypted data transport: encrypted packets

The same as `netcode`.

### 2. Encrypted data transport: unencrypted packets

**Packet sender**:

- Concatenate each packet's raw data with the protocol id in little-endian bytes. The resulting packet will be `prefix_byte | sequence | data | 8 bytes of garbage | protocol_id`. The garbage bytes is filler so encoded packets will have the same length as encrypted packets.
- Send the packet.

**Packet receiver**:

- Parse the protocol id from the end of each packet as little-endian bytes and discard the garbage bytes. Error if it does not match the expected id.
- Read the remainder of the packet as raw bytes. The remaining bytes will be `prefix_byte | sequence | data`.


## Multiple Server Sockets

`Netcode` was designed for servers with a single UDP socket that handles all client connections. However, servers can be hooked up to other socket types, such as WebTransport. It is useful if a server can simultaneously handle client connections that originate from different socket types (e.g. UDP for native clients and WebTransport for web clients).

**Client addresses in the server**

Since different socket types may have overlapping client ID spaces (e.g. `SocketAddr` for UDP, and a client index for WebTransport), it is necessary to 'domain-separate' addresses received from different sockets.

- Include a socket ID wherever `SocketAddr` is currently used in the server. This includes wherever the `SocketAddr` is used as a key for identifying existing and pending connections.

**Connect token**

- Add a 1-byte socket ID between `xnonces` and `server_addresses`.

**Private connect token**

- Add a 1-byte socket ID between `timeout_seconds` and `server_addresses`.

**Socket config in the server**

- Servers store a socket config for each distinct socket. Socket ids index into the socket config list.
    - `insecure`: Boolean value indicates if the socket is unencrypted. If true then packets will be encrypted (see the **Optional Encryption** extension).
    - `public_addresses`: Public address list associated with this socket. Stored as a list of `SocketAddr`, however sockets can overload the `SocketAddr` bytes to record custom socket address information.
- Use the socket id associated with clients and client packets to select the appropriate socket config for managing client connections.
