use bytes::Bytes;
use error::ProtocolError;
use message::{
    AgentTcpPayload, AgentUdpPayload, NetAddress, PayloadType, ProxyTcpInitResponseFailureReason,
    ProxyTcpInitResponseStatus, ProxyTcpPayload, ProxyUdpPayload, UnwrappedAgentTcpMessage,
    UnwrappedAgentUdpMessage, UnwrappedProxyTcpMessage, UnwrappedProxyUdpMessage, WrapperMessage,
};
use uuid::Uuid;

pub mod error;
pub mod message;

/// Generate a 32 length bytes
fn random_32_bytes() -> Bytes {
    let mut result = Vec::new();
    result.extend_from_slice(Uuid::new_v4().as_bytes());
    result.extend_from_slice(Uuid::new_v4().as_bytes());
    result.into()
}

pub fn new_agent_tcp_init_request(
    tunnel_id: String,
    user_token: String,
    src_address: NetAddress,
    dst_address: NetAddress,
) -> Result<WrapperMessage, ProtocolError> {
    let payload = AgentTcpPayload::InitRequest {
        src_address,
        dst_address,
    };
    let payload = payload.try_into()?;
    Ok(WrapperMessage {
        tunnel_id: Some(tunnel_id),
        message_id: Uuid::new_v4().to_string(),
        user_token,
        encryption: message::Encryption::Aes(random_32_bytes()),
        payload_type: PayloadType::Tcp,
        payload,
    })
}

pub fn new_agent_tcp_data(
    tunnel_id: String,
    user_token: String,
    data: Bytes,
) -> Result<WrapperMessage, ProtocolError> {
    let payload = AgentTcpPayload::Data { data };
    let payload = payload.try_into()?;
    Ok(WrapperMessage {
        tunnel_id: Some(tunnel_id),
        message_id: Uuid::new_v4().to_string(),
        user_token,
        encryption: message::Encryption::Aes(random_32_bytes()),
        payload_type: PayloadType::Tcp,
        payload,
    })
}

pub fn new_agent_tcp_close_request(
    tunnel_id: String,
    user_token: String,
) -> Result<WrapperMessage, ProtocolError> {
    let payload = AgentTcpPayload::CloseRequest;
    let payload = payload.try_into()?;
    Ok(WrapperMessage {
        tunnel_id: Some(tunnel_id),
        message_id: Uuid::new_v4().to_string(),
        user_token,
        encryption: message::Encryption::Aes(random_32_bytes()),
        payload_type: PayloadType::Tcp,
        payload,
    })
}

pub fn new_agent_udp_data(
    tunnel_id: String,
    user_token: String,
    src_address: NetAddress,
    dst_address: NetAddress,
    data: Bytes,
    expect_response: bool,
) -> Result<WrapperMessage, ProtocolError> {
    let payload = AgentUdpPayload {
        data,
        src_address,
        dst_address,
        expect_response,
    };
    let payload = payload.try_into()?;
    Ok(WrapperMessage {
        tunnel_id: Some(tunnel_id),
        message_id: Uuid::new_v4().to_string(),
        user_token,
        encryption: message::Encryption::Aes(random_32_bytes()),
        payload_type: PayloadType::Udp,
        payload,
    })
}

pub fn new_proxy_tcp_init_success_response(
    tunnel_id: String,
    user_token: String,
    src_address: NetAddress,
    dst_address: NetAddress,
) -> Result<WrapperMessage, ProtocolError> {
    let response_status = ProxyTcpInitResponseStatus::Success {
        src_address,
        dst_address,
    };
    let payload = ProxyTcpPayload::InitResponse(response_status).try_into()?;
    Ok(WrapperMessage {
        tunnel_id: Some(tunnel_id),
        message_id: Uuid::new_v4().to_string(),
        user_token,
        encryption: message::Encryption::Aes(random_32_bytes()),
        payload_type: PayloadType::Tcp,
        payload,
    })
}

pub fn new_proxy_tcp_init_failure_response(
    tunnel_id: String,
    user_token: String,
    src_address: NetAddress,
    dst_address: NetAddress,
) -> Result<WrapperMessage, ProtocolError> {
    let response_status = ProxyTcpInitResponseStatus::Failure {
        src_address,
        dst_address,
        reason: ProxyTcpInitResponseFailureReason::Other,
    };
    let payload = ProxyTcpPayload::InitResponse(response_status).try_into()?;
    Ok(WrapperMessage {
        tunnel_id: Some(tunnel_id),
        message_id: Uuid::new_v4().to_string(),
        user_token,
        encryption: message::Encryption::Aes(random_32_bytes()),
        payload_type: PayloadType::Tcp,
        payload,
    })
}

pub fn new_proxy_tcp_data(
    user_token: String,
    tunnel_id: String,
    data: Bytes,
) -> Result<WrapperMessage, ProtocolError> {
    let payload = ProxyTcpPayload::Data { data };
    let payload = payload.try_into()?;
    Ok(WrapperMessage {
        tunnel_id: Some(tunnel_id),
        message_id: Uuid::new_v4().to_string(),
        user_token,
        encryption: message::Encryption::Aes(random_32_bytes()),
        payload_type: PayloadType::Tcp,
        payload,
    })
}

pub fn new_proxy_tcp_close_request(
    user_token: String,
    tunnel_id: String,
) -> Result<WrapperMessage, ProtocolError> {
    let payload = ProxyTcpPayload::CloseRequest;
    let payload = payload.try_into()?;
    Ok(WrapperMessage {
        tunnel_id: Some(tunnel_id),
        message_id: Uuid::new_v4().to_string(),
        user_token,
        encryption: message::Encryption::Aes(random_32_bytes()),
        payload_type: PayloadType::Tcp,
        payload,
    })
}

pub fn new_proxy_udp_data(
    tunnel_id: String,
    user_token: String,
    src_address: NetAddress,
    dst_address: NetAddress,
    data: Bytes,
) -> Result<WrapperMessage, ProtocolError> {
    let payload = ProxyUdpPayload {
        data,
        src_address,
        dst_address,
    };
    let payload = payload.try_into()?;
    Ok(WrapperMessage {
        tunnel_id: Some(tunnel_id),
        message_id: Uuid::new_v4().to_string(),
        user_token,
        encryption: message::Encryption::Aes(random_32_bytes()),
        payload_type: PayloadType::Udp,
        payload,
    })
}

pub fn unwrap_agent_tcp_message(
    message: WrapperMessage,
) -> Result<UnwrappedAgentTcpMessage, ProtocolError> {
    let WrapperMessage {
        message_id,
        user_token,
        payload_type,
        payload,
        encryption,
        tunnel_id,
        ..
    } = message;
    if payload_type != PayloadType::Tcp {
        return Err(ProtocolError::Other(format!("Fail to parse wrapper message [{message_id}] for user [{user_token}] in tunnel [{tunnel_id:?}] to agent tcp payload because of the payload type is not TCP: {payload_type:?}")));
    }
    let agent_tcp_payload: AgentTcpPayload = payload.try_into()?;
    Ok(UnwrappedAgentTcpMessage {
        tunnel_id,
        message_id,
        user_token,
        encryption,
        payload_type,
        payload: agent_tcp_payload,
    })
}

pub fn unwrap_agent_udp_message(
    message: WrapperMessage,
) -> Result<UnwrappedAgentUdpMessage, ProtocolError> {
    let WrapperMessage {
        message_id,
        tunnel_id,
        user_token,
        payload_type,
        payload,
        encryption,
        ..
    } = message;
    if payload_type != PayloadType::Udp {
        return Err(ProtocolError::Other(format!("Fail to parse wrapper message [{message_id}] for user [{user_token}] in tunnel [{tunnel_id:?}] to agent udp payload because of the payload type is not UDP: {payload_type:?}")));
    }
    let agent_udp_payload: AgentUdpPayload = payload.try_into()?;
    Ok(UnwrappedAgentUdpMessage {
        message_id,
        tunnel_id,
        user_token,
        encryption,
        payload_type,
        payload: agent_udp_payload,
    })
}

pub fn unwrap_proxy_tcp_message(
    message: WrapperMessage,
) -> Result<UnwrappedProxyTcpMessage, ProtocolError> {
    let WrapperMessage {
        message_id,
        tunnel_id,
        user_token,
        payload_type,
        payload,
        encryption,
        ..
    } = message;
    if payload_type != PayloadType::Tcp {
        return Err(ProtocolError::Other(format!("Fail to parse wrapper message [{message_id}] for user [{user_token}] in tunnel [{tunnel_id:?}] to proxy tcp payload because of the payload type is not TCP: {payload_type:?}")));
    }
    let proxy_tcp_payload: ProxyTcpPayload = payload.try_into()?;
    Ok(UnwrappedProxyTcpMessage {
        message_id,
        tunnel_id,
        user_token,
        encryption,
        payload_type,
        payload: proxy_tcp_payload,
    })
}

pub fn unwrap_proxy_udp_payload(
    message: WrapperMessage,
) -> Result<UnwrappedProxyUdpMessage, ProtocolError> {
    let WrapperMessage {
        message_id,
        tunnel_id,
        user_token,
        payload_type,
        payload,
        encryption,
        ..
    } = message;
    if payload_type != PayloadType::Udp {
        return Err(ProtocolError::Other(format!("Fail to parse wrapper message [{message_id}] for user [{user_token}] in tunnel [{tunnel_id:?}] to proxy udp payload because of the payload type is not UDP: {payload_type:?}")));
    }
    let proxy_udp_payload: ProxyUdpPayload = payload.try_into()?;
    Ok(UnwrappedProxyUdpMessage {
        message_id,
        tunnel_id,
        user_token,
        encryption,
        payload_type,
        payload: proxy_udp_payload,
    })
}
