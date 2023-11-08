mod payload;

use crate::{error::ProtocolError, message::Encryption};
use bytes::Bytes;
use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};

pub use self::payload::AgentMessagePayload;

/// The agent message
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Constructor)]
pub struct AgentMessage {
    /// The unique id to idenfiy this message
    pub unique_id: String,
    /// The user token
    pub user_token: String,
    /// The encryption of the payload used for this message
    pub encryption: Encryption,
    /// The message payload
    pub payload: Bytes,
}

impl TryFrom<Bytes> for AgentMessage {
    type Error = ProtocolError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        bincode::deserialize(&value).map_err(ProtocolError::Deserialize)
    }
}

impl TryFrom<AgentMessage> for Bytes {
    type Error = ProtocolError;

    fn try_from(value: AgentMessage) -> Result<Self, Self::Error> {
        bincode::serialize(&value)
            .map(Bytes::from)
            .map_err(ProtocolError::Serialize)
    }
}
