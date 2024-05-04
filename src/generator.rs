use crate::error::ProtocolError;
use bytes::Bytes;
use uuid::Uuid;

use crate::message::payload::tcp::{AgentTcpPayload, ProxyTcpPayload};
use crate::message::payload::{
    tcp::ProxyTcpInitResult,
    udp::{AgentUdpPayload, ProxyUdpPayload},
};
use crate::message::values::address::PpaassUnifiedAddress;
use crate::message::values::encryption::PpaassMessagePayloadEncryption;
use crate::message::{
    PpaassAgentMessage, PpaassAgentMessagePayload, PpaassProxyMessage, PpaassProxyMessagePayload,
};

pub struct PpaassMessageGenerator;

impl PpaassMessageGenerator {
    /// Generate the agent tcp init message
    pub fn generate_agent_tcp_init_message(
        user_token: String,
        src_address: PpaassUnifiedAddress,
        dst_address: PpaassUnifiedAddress,
        encryption: PpaassMessagePayloadEncryption,
    ) -> Result<PpaassAgentMessage, ProtocolError> {
        let payload = PpaassAgentMessagePayload::Tcp(AgentTcpPayload::Init {
            src_address,
            dst_address,
        });
        let message =
            PpaassAgentMessage::new(Uuid::new_v4().to_string(), user_token, encryption, payload);
        Ok(message)
    }

    /// Generate the proxy tcp init message
    pub fn generate_proxy_tcp_init_message(
        user_token: String,
        src_address: PpaassUnifiedAddress,
        dst_address: PpaassUnifiedAddress,
        encryption: PpaassMessagePayloadEncryption,
        result: ProxyTcpInitResult,
    ) -> Result<PpaassProxyMessage, ProtocolError> {
        let payload = PpaassProxyMessagePayload::Tcp(ProxyTcpPayload::Init {
            src_address,
            dst_address,
            result,
        });
        let message = PpaassProxyMessage::new(
            Uuid::new_v4().to_string(),
            user_token.to_string(),
            encryption,
            payload,
        );
        Ok(message)
    }

    /// Generate the agent tcp data message
    pub fn generate_agent_tcp_data_message(
        user_token: String,
        encryption: PpaassMessagePayloadEncryption,
        data: Bytes,
    ) -> Result<PpaassAgentMessage, ProtocolError> {
        let payload = PpaassAgentMessagePayload::Tcp(AgentTcpPayload::Data { content: data });
        let message = PpaassAgentMessage::new(
            Uuid::new_v4().to_string(),
            user_token.to_string(),
            encryption,
            payload,
        );
        Ok(message)
    }

    /// Generate the proxy tcp data message
    pub fn generate_proxy_tcp_data_message(
        user_token: String,
        encryption: PpaassMessagePayloadEncryption,
        data: Bytes,
    ) -> Result<PpaassProxyMessage, ProtocolError> {
        let payload = PpaassProxyMessagePayload::Tcp(ProxyTcpPayload::Data { content: data });
        let message = PpaassProxyMessage::new(
            Uuid::new_v4().to_string(),
            user_token.to_string(),
            encryption,
            payload,
        );
        Ok(message)
    }

    /// Generate the agent udp data message
    pub fn generate_agent_udp_data_message(
        user_token: String,
        encryption: PpaassMessagePayloadEncryption,
        src_address: PpaassUnifiedAddress,
        dst_address: PpaassUnifiedAddress,
        data: Bytes,
    ) -> Result<PpaassAgentMessage, ProtocolError> {
        let payload = PpaassAgentMessagePayload::Udp(AgentUdpPayload::Data {
            src_address,
            dst_address,
            data,
        });
        let message = PpaassAgentMessage::new(
            Uuid::new_v4().to_string(),
            user_token.to_string(),
            encryption,
            payload,
        );
        Ok(message)
    }

    /// Generate the proxy udp data message
    pub fn generate_proxy_udp_data_message(
        user_token: String,
        encryption: PpaassMessagePayloadEncryption,
        src_address: PpaassUnifiedAddress,
        dst_address: PpaassUnifiedAddress,
        data: Bytes,
    ) -> Result<PpaassProxyMessage, ProtocolError> {
        let payload = PpaassProxyMessagePayload::Udp(ProxyUdpPayload {
            src_address,
            dst_address,
            data,
        });
        let message = PpaassProxyMessage::new(
            Uuid::new_v4().to_string(),
            user_token.to_string(),
            encryption,
            payload,
        );
        Ok(message)
    }
}
