use bytes::Bytes;
use error::ProtocolError;
use message::{
    AgentTcpPayload, AgentUdpPayload, NetAddress, PayloadType, ProxyTcpInitResponseFailureReason,
    ProxyTcpInitResponseStatus, ProxyTcpPayload, ProxyUdpPayload, WrapperMessage,
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
        unique_id: Uuid::new_v4().to_string(),
        user_token,
        encryption: message::Encryption::Aes(random_32_bytes()),
        payload_type: PayloadType::Tcp,
        payload,
    })
}

pub fn new_agent_tcp_data(
    user_token: String,
    connection_id: String,
    data: Bytes,
) -> Result<WrapperMessage, ProtocolError> {
    let payload = AgentTcpPayload::Data {
        connection_id,
        data,
    };
    let payload = payload.try_into()?;
    Ok(WrapperMessage {
        unique_id: Uuid::new_v4().to_string(),
        user_token,
        encryption: message::Encryption::Aes(random_32_bytes()),
        payload_type: PayloadType::Tcp,
        payload,
    })
}

pub fn new_agent_udp_data(
    user_token: String,
    connection_id: String,
    src_address: NetAddress,
    dst_address: NetAddress,
    data: Bytes,
    expect_response: bool,
) -> Result<WrapperMessage, ProtocolError> {
    let payload = AgentUdpPayload {
        connection_id,
        data,
        src_address,
        dst_address,
        expect_response,
    };
    let payload = payload.try_into()?;
    Ok(WrapperMessage {
        unique_id: Uuid::new_v4().to_string(),
        user_token,
        encryption: message::Encryption::Aes(random_32_bytes()),
        payload_type: PayloadType::Udp,
        payload,
    })
}

pub fn new_proxy_tcp_init_success_response(
    connection_id: String,
    user_token: String,
    src_address: NetAddress,
    dst_address: NetAddress,
) -> Result<WrapperMessage, ProtocolError> {
    let response_status = ProxyTcpInitResponseStatus::Success {
        connection_id,
        src_address,
        dst_address,
    };
    let payload = ProxyTcpPayload::InitResponse(response_status).try_into()?;
    Ok(WrapperMessage {
        unique_id: Uuid::new_v4().to_string(),
        user_token,
        encryption: message::Encryption::Aes(random_32_bytes()),
        payload_type: PayloadType::Tcp,
        payload,
    })
}

pub fn new_proxy_tcp_init_failure_response(
    connection_id: String,
    user_token: String,
    src_address: NetAddress,
    dst_address: NetAddress,
) -> Result<WrapperMessage, ProtocolError> {
    let response_status = ProxyTcpInitResponseStatus::Failure {
        connection_id,
        src_address,
        dst_address,
        reason: ProxyTcpInitResponseFailureReason::Other,
    };
    let payload = ProxyTcpPayload::InitResponse(response_status).try_into()?;
    Ok(WrapperMessage {
        unique_id: Uuid::new_v4().to_string(),
        user_token,
        encryption: message::Encryption::Aes(random_32_bytes()),
        payload_type: PayloadType::Tcp,
        payload,
    })
}

pub fn new_proxy_tcp_data(
    user_token: String,
    connection_id: String,
    data: Bytes,
) -> Result<WrapperMessage, ProtocolError> {
    let payload = ProxyTcpPayload::Data {
        connection_id,
        data,
    };
    let payload = payload.try_into()?;
    Ok(WrapperMessage {
        unique_id: Uuid::new_v4().to_string(),
        user_token,
        encryption: message::Encryption::Aes(random_32_bytes()),
        payload_type: PayloadType::Tcp,
        payload,
    })
}

pub fn new_proxy_udp_data(
    user_token: String,
    connection_id: String,
    src_address: NetAddress,
    dst_address: NetAddress,
    data: Bytes,
) -> Result<WrapperMessage, ProtocolError> {
    let payload = ProxyUdpPayload {
        connection_id,
        data,
        src_address,
        dst_address,
    };
    let payload = payload.try_into()?;
    Ok(WrapperMessage {
        unique_id: Uuid::new_v4().to_string(),
        user_token,
        encryption: message::Encryption::Aes(random_32_bytes()),
        payload_type: PayloadType::Udp,
        payload,
    })
}

pub fn unwrap_agent_tcp_payload(message: WrapperMessage) -> Result<AgentTcpPayload, ProtocolError> {
    let WrapperMessage {
        unique_id,
        user_token,
        payload_type,
        payload,
        ..
    } = message;
    if payload_type != PayloadType::Tcp {
        return Err(ProtocolError::Other(format!("Fail to parse wrapper message [{unique_id}] for user [{user_token}] to agent tcp payload because of the payload type is not TCP: {payload_type:?}")));
    }
    let payload: AgentTcpPayload = payload.try_into()?;
    Ok(payload)
}

pub fn unwrap_agent_udp_payload(message: WrapperMessage) -> Result<AgentUdpPayload, ProtocolError> {
    let WrapperMessage {
        unique_id,
        user_token,
        payload_type,
        payload,
        ..
    } = message;
    if payload_type != PayloadType::Udp {
        return Err(ProtocolError::Other(format!("Fail to parse wrapper message [{unique_id}] for user [{user_token}] to agent tcp payload because of the payload type is not UDP: {payload_type:?}")));
    }
    let payload: AgentUdpPayload = payload.try_into()?;
    Ok(payload)
}

pub fn unwrap_proxy_tcp_payload(message: WrapperMessage) -> Result<ProxyTcpPayload, ProtocolError> {
    let WrapperMessage {
        unique_id,
        user_token,
        payload_type,
        payload,
        ..
    } = message;
    if payload_type != PayloadType::Tcp {
        return Err(ProtocolError::Other(format!("Fail to parse wrapper message [{unique_id}] for user [{user_token}] to proxy tcp payload because of the payload type is not TCP: {payload_type:?}")));
    }
    let payload: ProxyTcpPayload = payload.try_into()?;
    Ok(payload)
}

pub fn unwrap_proxy_udp_payload(message: WrapperMessage) -> Result<ProxyUdpPayload, ProtocolError> {
    let WrapperMessage {
        unique_id,
        user_token,
        payload_type,
        payload,
        ..
    } = message;
    if payload_type != PayloadType::Udp {
        return Err(ProtocolError::Other(format!("Fail to parse wrapper message [{unique_id}] for user [{user_token}] to proxy tcp payload because of the payload type is not UDP: {payload_type:?}")));
    }
    let payload: ProxyUdpPayload = payload.try_into()?;
    Ok(payload)
}
