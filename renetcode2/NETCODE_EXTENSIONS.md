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
