use bytes::Bytes;
use serde_derive::{Deserialize, Serialize};

use crate::NetAddress;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum AgentTcpPayload {
    Connect {
        src_address: NetAddress,
        dst_address: NetAddress,
    },
    Data(Bytes),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum AgentMessagePayload {
    Tcp(AgentTcpPayload),
    Udp {
        src_address: NetAddress,
        dst_address: NetAddress,
        data: Bytes,
        expect_response: bool,
    },
}
