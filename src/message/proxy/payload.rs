use bytes::Bytes;
use serde_derive::{Deserialize, Serialize};

use crate::NetAddress;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ProxyTcpPayload {
    Connect {
        src_address: NetAddress,
        dst_address: NetAddress,
    },
    Data(Bytes),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProxyMessagePayload {
    Tcp(ProxyTcpPayload),
    Udp {
        src_address: NetAddress,
        dst_address: NetAddress,
        data: Bytes,
    },
}
