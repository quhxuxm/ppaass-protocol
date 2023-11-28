use super::{
    AgentTcpPayload, AgentUdpPayload, Encryption, PayloadType, ProxyTcpPayload, ProxyUdpPayload,
};

pub struct UnwrappedAgentTcpMessage {
    /// The tunnel id of the
    pub tunnel_id: String,
    /// The message id to idenfiy this message
    pub message_id: String,
    /// The user token
    pub user_token: String,
    /// The encryption of the payload used for this message
    pub encryption: Encryption,
    /// The type of the payload of this message
    pub payload_type: PayloadType,
    /// The agent tcp payload
    pub payload: AgentTcpPayload,
}

pub struct UnwrappedAgentUdpMessage {
    /// The tunnel id of the
    pub tunnel_id: String,
    /// The message id to idenfiy this message
    pub message_id: String,
    /// The user token
    pub user_token: String,
    /// The encryption of the payload used for this message
    pub encryption: Encryption,
    /// The type of the payload of this message
    pub payload_type: PayloadType,
    /// The agent udp payload
    pub payload: AgentUdpPayload,
}

pub struct UnwrappedProxyTcpMessage {
    /// The tunnel id of the
    pub tunnel_id: String,
    /// The message id to idenfiy this message
    pub message_id: String,
    /// The user token
    pub user_token: String,
    /// The encryption of the payload used for this message
    pub encryption: Encryption,
    /// The type of the payload of this message
    pub payload_type: PayloadType,
    /// The proxy tcp payload
    pub payload: ProxyTcpPayload,
}

pub struct UnwrappedProxyUdpMessage {
    /// The tunnel id of the
    pub tunnel_id: String,
    /// The message id to idenfiy this message
    pub message_id: String,
    /// The user token
    pub user_token: String,
    /// The encryption of the payload used for this message
    pub encryption: Encryption,
    /// The type of the payload of this message
    pub payload_type: PayloadType,
    /// The proxy udp payload
    pub payload: ProxyUdpPayload,
}
