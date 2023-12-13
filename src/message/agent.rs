use crate::error::ProtocolError;
use crate::make_as_bytes;
use crate::values::address::UnifiedNetAddress;
use crate::values::security::SecureInfo;
use bytes::Bytes;
use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub enum AgentMessagePayload {
    InitTunnelCommand(InitTunnelCommand),
    RelayData(RelayData),
    CloseTunnelCommand(CloseTunnelCommand),
}

#[derive(Debug)]
pub struct AgentMessage {
    /// The request id
    pub message_id: String,
    /// The secure information
    pub secure_info: SecureInfo,
    /// The payload of the wrapper message
    pub payload: AgentMessagePayload,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EncodedAgentMessagePayload {
    InitTunnelCommand(Bytes),
    RelayData(Bytes),
    CloseTunnelCommand(Bytes),
}

make_as_bytes! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    struct EncodedAgentMessage {
        /// The request id
        message_id: String,
        /// The secure information
        secure_info: SecureInfo,
        /// The payload of the wrapper message
        payload: EncodedAgentMessagePayload
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InitTunnelCommand {
    Tcp {
        /// The source address of this request
        src_address: UnifiedNetAddress,
        /// The destination address of this request
        dst_address: UnifiedNetAddress,
        /// The agent edge id
        agent_edge_id: String,
    },
    Udp {
        /// The source address of this request
        src_address: UnifiedNetAddress,
        /// The agent edge id
        agent_edge_id: String,
    },
}

make_as_bytes! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    struct RelayData {
        /// The agent edge id
        agent_edge_id: String,
        /// The agent edge id
        proxy_edge_id: String,
        /// The source address
        src_address: UnifiedNetAddress,
        /// The destination address
        dst_address: UnifiedNetAddress,
        /// The relay data
        data: Bytes
    }
}

make_as_bytes! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    struct CloseTunnelCommand {
        /// The agent edge id
        agent_edge_id: String,
        /// The agent edge id
        proxy_edge_id: String,
        /// The source address
        src_address: UnifiedNetAddress,
        /// The destination address
        dst_address: UnifiedNetAddress,
    }
}
