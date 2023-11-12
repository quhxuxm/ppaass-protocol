use bytes::Bytes;
use serde_derive::{Deserialize, Serialize};

use crate::{error::ProtocolError, message::NetAddress};

/// The tcp payload in agent message
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum AgentTcpPayload {
    /// Tcp flow will do connect first
    InitRequest {
        /// The source address
        src_address: NetAddress,
        /// The destination address
        dst_address: NetAddress,
    },
    /// After connect the relay process will happen
    Data {
        /// The connection id between agent and proxy
        connection_id: String,
        /// The data relay from agent to proxy
        data: Bytes,
    },
}

/// The udp payload in agent message
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AgentUdpPayload {
    /// The connection id between agent and proxy
    connection_id: String,
    /// The source address
    src_address: NetAddress,
    /// The destination address
    dst_address: NetAddress,
    /// The data relay from agent to proxy
    data: Bytes,
    /// If the udp message need a response from proxy
    expect_response: bool,
}

impl TryFrom<Bytes> for AgentTcpPayload {
    type Error = ProtocolError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        bincode::deserialize(&value).map_err(ProtocolError::Deserialize)
    }
}

impl TryFrom<AgentTcpPayload> for Bytes {
    type Error = ProtocolError;

    fn try_from(value: AgentTcpPayload) -> Result<Self, Self::Error> {
        bincode::serialize(&value)
            .map(Bytes::from)
            .map_err(ProtocolError::Serialize)
    }
}

impl TryFrom<Bytes> for AgentUdpPayload {
    type Error = ProtocolError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        bincode::deserialize(&value).map_err(ProtocolError::Deserialize)
    }
}

impl TryFrom<AgentUdpPayload> for Bytes {
    type Error = ProtocolError;

    fn try_from(value: AgentUdpPayload) -> Result<Self, Self::Error> {
        bincode::serialize(&value)
            .map(Bytes::from)
            .map_err(ProtocolError::Serialize)
    }
}
