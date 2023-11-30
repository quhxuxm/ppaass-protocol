use crate::error::ProtocolError;
use crate::make_as_bytes;
use crate::values::address::NetAddress;
use crate::values::security::SecureInfo;
use crate::values::tunnel::Tunnel;
use bytes::Bytes;
use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};

make_as_bytes! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    struct WrapperMessage {
        /// The request id
        message_id: String,
        /// The secure information
        secure_info: SecureInfo,
        /// The payload of the wrapper message
        payload: Bytes
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
    struct InitTunnelResult {
        /// The source address
        src_address: NetAddress,
        /// The destination address
        dst_address: NetAddress,
        /// The tunnel initialized on proxy side
        tunnel: Tunnel,
    }
}

make_as_bytes! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    struct RelayDataCommand {
        /// The source address
        src_address: NetAddress,
        /// The destination address
        dst_address: NetAddress,
        /// The tunnel initialized on proxy side
        tunnel: Tunnel,
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
        /// The tunnel initialized on proxy side
        tunnel: Tunnel,
    }
}
