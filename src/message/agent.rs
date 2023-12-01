use crate::error::ProtocolError;
use crate::make_as_bytes;
use crate::values::address::NetAddress;
use crate::values::security::SecureInfo;
use crate::values::tunnel::Tunnel;
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
    /// The tunnel
    pub tunnel: Tunnel,
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
        /// The tunnel
        tunnel: Tunnel,
        /// The payload of the wrapper message
        payload: EncodedAgentMessagePayload
    }
}

make_as_bytes! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    struct InitTunnelCommand {
        /// The source address of this request
        src_address: NetAddress,
        /// The destination address of this request
        dst_address: NetAddress,
    }
}

make_as_bytes! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    struct RelayData {
        /// The source address
        src_address: NetAddress,
        /// The destination address
        dst_address: NetAddress,
        /// The relay data
        data: Bytes
    }
}

make_as_bytes! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    struct CloseTunnelCommand {
        /// The source address
        src_address: NetAddress,
        /// The destination address
        dst_address: NetAddress,
    }
}
