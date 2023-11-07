use bytes::Bytes;
use serde_derive::{Deserialize, Serialize};

use crate::NetAddress;

/// The tcp payload in proxy message
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ProxyTcpPayload {
    /// Tcp flow will do connect first
    Connect {
        src_address: NetAddress,
        dst_address: NetAddress,
    },
    /// After connect the relay process will happen
    Data(Bytes),
}

/// The proxy message payload
#[derive(Serialize, Deserialize, Debug)]
pub enum ProxyMessagePayload {
    /// Tcp payload
    Tcp(ProxyTcpPayload),
    /// Udp payload
    Udp {
        src_address: NetAddress,
        dst_address: NetAddress,
        data: Bytes,
    },
}
