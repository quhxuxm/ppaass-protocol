use bytes::Bytes;
use serde_derive::{Deserialize, Serialize};

use crate::NetAddress;

/// The tcp payload in agent message
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum AgentTcpPayload {
    /// Tcp flow will do connect first
    Connect {
        src_address: NetAddress,
        dst_address: NetAddress,
    },
    /// After connect the relay process will happen
    Data(Bytes),
}

/// The agent message payload
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum AgentMessagePayload {
    /// Tcp payload
    Tcp(AgentTcpPayload),
    /// Udp payload
    Udp {
        src_address: NetAddress,
        dst_address: NetAddress,
        data: Bytes,
        expect_response: bool,
    },
}
