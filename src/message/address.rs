use derive_more::Display;
use serde_derive::{Deserialize, Serialize};

use std::net::SocketAddr;

/// The net address for ppaass protocol which will transfer between
/// agent and proxy
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Display)]
pub enum NetAddress {
    /// The ip address, including support for v4 and v6
    #[display(fmt = "{:?}", _0)]
    Ip(SocketAddr),
    /// The domain name address
    #[display(fmt = "{}:{}", host, port)]
    Domain { host: String, port: u16 },
}

impl PartialEq for NetAddress {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NetAddress::Ip(self_socket_addr), NetAddress::Ip(other_socket_addr)) => {
                self_socket_addr.eq(other_socket_addr)
            }
            (
                NetAddress::Domain {
                    host: self_host,
                    port: self_port,
                },
                NetAddress::Domain {
                    host: other_host,
                    port: other_port,
                },
            ) => self_host.eq(other_host) && self_port.eq(other_port),
            _ => false,
        }
    }
}
