use crate::{error::ProtocolError, message::NetAddress};
use bytes::Bytes;
use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};

/// The tcp payload in proxy message
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ProxyTcpPayload {
    /// Tcp flow will do connect first
    InitResponse {
        /// The connection id between agent and proxy
        connection_id: String,
        /// The source address
        src_address: NetAddress,
        /// The destination address
        dst_address: NetAddress,
    },
    /// After connect the relay process will happen
    Data {
        /// The connection id between agent and proxy
        connection_id: String,
        /// The data relay from proxy to agent
        data: Bytes,
    },
}

/// The udp payload in agent message
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Constructor)]
pub struct ProxyUdpPayload {
    /// The connection id between agent and proxy
    connection_id: String,
    /// The source address
    src_address: NetAddress,
    /// The destination address
    dst_address: NetAddress,
    /// The data relay from proxy to agent
    data: Bytes,
}

impl TryFrom<Bytes> for ProxyTcpPayload {
    type Error = ProtocolError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        bincode::deserialize(&value).map_err(ProtocolError::Deserialize)
    }
}

impl TryFrom<ProxyTcpPayload> for Bytes {
    type Error = ProtocolError;

    fn try_from(value: ProxyTcpPayload) -> Result<Self, Self::Error> {
        bincode::serialize(&value)
            .map(Bytes::from)
            .map_err(ProtocolError::Serialize)
    }
}

impl TryFrom<Bytes> for ProxyUdpPayload {
    type Error = ProtocolError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        bincode::deserialize(&value).map_err(ProtocolError::Deserialize)
    }
}

impl TryFrom<ProxyUdpPayload> for Bytes {
    type Error = ProtocolError;

    fn try_from(value: ProxyUdpPayload) -> Result<Self, Self::Error> {
        bincode::serialize(&value)
            .map(Bytes::from)
            .map_err(ProtocolError::Serialize)
    }
}
