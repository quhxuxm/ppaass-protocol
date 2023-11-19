use super::{
    AgentTcpPayload, AgentUdpPayload, Encryption, PayloadType, ProxyTcpPayload, ProxyUdpPayload,
};

pub struct UnwrappedAgentTcpPayload {
    /// The unique id to idenfiy this message
    pub unique_id: String,
    /// The user token
    pub user_token: String,
    /// The encryption of the payload used for this message
    pub encryption: Encryption,
    /// The type of the payload of this message
    pub payload_type: PayloadType,
    /// The agent tcp payload
    pub payload: AgentTcpPayload,
}

pub struct UnwrappedAgentUdpPayload {
    /// The unique id to idenfiy this message
    pub unique_id: String,
    /// The user token
    pub user_token: String,
    /// The encryption of the payload used for this message
    pub encryption: Encryption,
    /// The type of the payload of this message
    pub payload_type: PayloadType,
    /// The agent udp payload
    pub payload: AgentUdpPayload,
}

pub struct UnwrappedProxyTcpPayload {
    /// The unique id to idenfiy this message
    pub unique_id: String,
    /// The user token
    pub user_token: String,
    /// The encryption of the payload used for this message
    pub encryption: Encryption,
    /// The type of the payload of this message
    pub payload_type: PayloadType,
    /// The proxy tcp payload
    pub payload: ProxyTcpPayload,
}

pub struct UnwrappedProxyUdpPayload {
    /// The unique id to idenfiy this message
    pub unique_id: String,
    /// The user token
    pub user_token: String,
    /// The encryption of the payload used for this message
    pub encryption: Encryption,
    /// The type of the payload of this message
    pub payload_type: PayloadType,
    /// The proxy udp payload
    pub payload: ProxyUdpPayload,
}
