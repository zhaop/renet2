mod client;
mod crypto;
mod packet;
mod serialize;
mod server;
mod token;

const NETCODE_VERSION_INFO: &[u8; 13] = b"NETCODE 1.02\0";

const NETCODE_ADDRESS_NONE: u8 = 0;
const NETCODE_ADDRESS_IPV4: u8 = 1;
const NETCODE_ADDRESS_IPV6: u8 = 2;

const NETCODE_CONNECT_TOKEN_PRIVATE_BYTES: usize = 1024;
const NETCODE_MAX_PACKET_SIZE: usize = 1200;

const NETCODE_KEY_BYTES: usize = 32;
const NETCODE_MAC_BYTES: usize = 16;
const NETCODE_USER_DATA_BYTES: usize = 256;
const NETCODE_CHALLENGE_TOKEN_BYTES: usize = 300;

const NETCODE_ADDITIONAL_DATA_SIZE: usize = 13 + 8 + 8;

const NETCODE_BUFFER_SIZE: usize = NETCODE_MAX_PACKET_SIZE + NETCODE_MAC_BYTES;
const NETCODE_TIMEOUT_SECONDS: i32 = 15;

const NETCODE_SEND_RATE: Duration = Duration::from_millis(100);

/*
     Encryption of the private connect token data is performed with the libsodium AEAD primitive crypto_aead_xchacha20poly1305_ietf_encrypt using the following binary data as the associated data:

       [version info] (13 bytes)       // "NETCODE 1.02" ASCII with null terminator.
        [protocol id] (uint64)          // 64 bit value unique to this particular game/application
        [expire timestamp] (uint64)     // 64 bit unix timestamp when this connect token expires

   The nonce used for encryption is a 24 bytes number that is randomly generated for every token.

    Encryption is performed on the first 1024 - 16 bytes in the buffer, leaving the last 16 bytes to store the HMAC:

    [encrypted private connect token] (1008 bytes)
    [hmac of encrypted private connect token] (16 bytes)
    Post encryption, this is referred to as the encrypted private connect token data.

    Together the public and private data form a connect token:

        [version info] (13 bytes)       // "NETCODE 1.02" ASCII with null terminator.
        [protocol id] (uint64)          // 64 bit value unique to this particular game/application
        [create timestamp] (uint64)     // 64 bit unix timestamp when this connect token was created
        [expire timestamp] (uint64)     // 64 bit unix timestamp when this connect token expires
        [connect token nonce] (24 bytes)
        [encrypted private connect token data] (1024 bytes)
        [timeout seconds] (uint32)      // timeout in seconds. negative values disable timeout (dev only)
        [num_server_addresses] (uint32) // in [1,32]
        <for each server address>
        {
            [address_type] (uint8) // value of 1 = IPv4 address, 2 = IPv6 address.
            <if IPV4 address>
            {
                // for a given IPv4 address: a.b.c.d:port
                [a] (uint8)
                [b] (uint8)
                [c] (uint8)
                [d] (uint8)
                [port] (uint16)
            }
            <else IPv6 address>
            {
                // for a given IPv6 address: [a:b:c:d:e:f:g:h]:port
                [a] (uint16)
                [b] (uint16)
                [c] (uint16)
                [d] (uint16)
                [e] (uint16)
                [f] (uint16)
                [g] (uint16)
                [h] (uint16)
                [port] (uint16)
            }
        }
        [client to server key] (32 bytes)
        [server to client key] (32 bytes)
        <zero pad to 2048 bytes>

    This data is variable size but for simplicity is written to a fixed size buffer of 2048 bytes. Unused bytes are zero padded.
*/

use std::time::Duration;