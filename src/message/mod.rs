pub mod payload;
pub mod values;

use crate::make_as_bytes;
use crate::message::payload::tcp::{AgentTcpPayload, ProxyTcpPayload};
use crate::message::payload::udp::{AgentUdpPayload, ProxyUdpPayload};

use bytes::Bytes;
use derive_more::Constructor;

use crate::error::ProtocolError;
use crate::message::values::encryption::PpaassMessagePayloadEncryption;
use serde_derive::{Deserialize, Serialize};

make_as_bytes! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    struct CodecPpaassMessage {
        message_id: String,
        user_token: String,
        encryption: PpaassMessagePayloadEncryption,
        payload: Bytes,
    }
}

make_as_bytes! {
    #[derive(Serialize, Deserialize, Debug)]
    pub enum PpaassAgentMessagePayload {
        Tcp(AgentTcpPayload),
        Udp(AgentUdpPayload),
    }
}

make_as_bytes! {
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    pub struct PpaassAgentMessage {
        pub message_id: String,
        pub user_token: String,
        pub encryption: PpaassMessagePayloadEncryption,
        pub payload: PpaassAgentMessagePayload,
    }
}

make_as_bytes! {
    #[derive(Serialize, Deserialize, Debug)]
    pub enum PpaassProxyMessagePayload {
        Tcp(ProxyTcpPayload),
        Udp(ProxyUdpPayload),
    }
}

make_as_bytes! {
    #[non_exhaustive]
    #[derive(Serialize, Deserialize, Debug, Constructor)]
    pub struct PpaassProxyMessage {
        pub message_id: String,
        pub user_token: String,
        pub encryption: PpaassMessagePayloadEncryption,
        pub payload: PpaassProxyMessagePayload,
    }
}
