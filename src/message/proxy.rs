use crate::error::ProtocolError;
use crate::make_as_bytes;
use crate::values::address::NetAddress;
use crate::values::security::SecureInfo;
use crate::values::tunnel::Tunnel;
use bytes::Bytes;
use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ProxyMessagePayload {
    InitTunnelResult(Bytes),
    RelayData(Bytes),
    CloseTunnelCommand(Bytes),
}

make_as_bytes! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    struct ProxyMessage {
        /// The request id
        message_id: String,
        /// The secure information
        secure_info: SecureInfo,
        /// The tunnel
        tunnel: Tunnel,
        /// The payload of the wrapper message
        payload: ProxyMessagePayload
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
