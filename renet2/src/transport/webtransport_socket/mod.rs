#[cfg(all(feature = "wt_client_transport", target_family = "wasm"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "wt_client_transport", target_family = "wasm"))))]
mod client;

#[cfg(all(feature = "wt_server_transport", not(target_family = "wasm")))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "wt_server_transport", not(target_family = "wasm")))))]
mod server;

#[cfg(any(feature = "wt_server_transport", feature = "wt_client_transport"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "wt_server_transport", feature = "wt_client_transport"))))]
mod utils;

#[cfg(all(feature = "wt_client_transport", target_family = "wasm"))]
pub use client::*;

#[cfg(all(feature = "wt_server_transport", not(target_family = "wasm")))]
pub use server::*;

#[cfg(any(feature = "wt_server_transport", feature = "wt_client_transport"))]
pub use utils::*;
