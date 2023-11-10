use bytes::Bytes;
use serde_derive::{Deserialize, Serialize};

use crate::{error::ProtocolError, message::NetAddress};

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

impl TryFrom<Bytes> for AgentMessagePayload {
    type Error = ProtocolError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        bincode::deserialize(&value).map_err(ProtocolError::Deserialize)
    }
}

impl TryFrom<AgentMessagePayload> for Bytes {
    type Error = ProtocolError;

    fn try_from(value: AgentMessagePayload) -> Result<Self, Self::Error> {
        bincode::serialize(&value)
            .map(Bytes::from)
            .map_err(ProtocolError::Serialize)
    }
}
