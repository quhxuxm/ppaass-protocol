use crate::error::ProtocolError;
use crate::make_as_protocol_message;
use crate::values::address::NetAddress;
use crate::values::security::SecureInfo;
use crate::values::tunnel::Tunnel;
use bytes::Bytes;
use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};

make_as_protocol_message! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    struct InitTunnelRequest {
        /// The request id
        request_id: String,
        /// The secure information
        secure_info: SecureInfo,
        /// The source address of this request
        src_address: NetAddress,
        /// The destination address of this request
        dst_address: NetAddress,
    }
}

make_as_protocol_message! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    struct InitTunnelResponse {
        /// The random id for this response
        response_id: String,
        /// The secure information of this message
        secure_info: SecureInfo,
        /// The source address
        src_address: NetAddress,
        /// The destination address
        dst_address: NetAddress,
        /// The tunnel initialized on proxy side
        tunnel: Tunnel,
    }
}

make_as_protocol_message! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    struct RelayData {
        /// The random id for this request
        message_id: String,
        /// The secure information of this message
        secure_info: SecureInfo,
        /// The source address
        src_address: NetAddress,
        /// The destination address
        dst_address: NetAddress,
        /// The tunnel initialized on proxy side
        tunnel: Tunnel,
    }
}

make_as_protocol_message! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    struct CloseTunnelRequest {
        /// The random id for this request
        message_id: String,
        /// The secure information of this message
        secure_info: SecureInfo,
        /// The source address
        src_address: NetAddress,
        /// The destination address
        dst_address: NetAddress,
        /// The tunnel initialized on proxy side
        tunnel: Tunnel,
    }
}
