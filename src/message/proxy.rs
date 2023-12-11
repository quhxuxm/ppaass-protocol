use crate::error::ProtocolError;
use crate::make_as_bytes;
use crate::values::address::NetAddress;
use crate::values::security::SecureInfo;
use crate::values::tunnel::TunnelInfo;
use bytes::Bytes;
use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ProxyMessagePayload {
    InitTunnelResult(InitTunnelResult),
    RelayData(RelayData),
    CloseTunnelCommand(CloseTunnelCommand),
}

#[derive(Debug)]
pub struct ProxyMessage {
    /// The request id
    pub message_id: String,
    /// The secure information
    pub secure_info: SecureInfo,
    /// The tunnel
    pub tunnel: TunnelInfo,
    /// The payload of the wrapper message
    pub payload: ProxyMessagePayload,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EncodedProxyMessagePayload {
    InitTunnelResult(Bytes),
    RelayData(Bytes),
    CloseTunnelCommand(Bytes),
}

make_as_bytes! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    struct EncodedProxyMessage {
        /// The request id
        message_id: String,
        /// The secure information
        secure_info: SecureInfo,
        /// The tunnel
        tunnel: TunnelInfo,
        /// The payload of the wrapper message
        payload: EncodedProxyMessagePayload
    }
}

make_as_bytes! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    struct InitTunnelResult {
        /// The source address
        src_address: NetAddress,
        /// The destination address
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
