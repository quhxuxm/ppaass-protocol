mod payload;

use bytes::Bytes;
use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};

use crate::{error::ProtocolError, message::Encryption};

pub use self::payload::ProxyMessagePayload;

/// The proxy message
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Constructor)]
pub struct ProxyMessage {
    /// The unique id to idenfiy this message
    pub unique_id: String,
    /// The user token
    pub user_token: String,
    /// The encryption of the payload used for this message
    pub encryption: Encryption,
    /// The message payload
    pub payload: Bytes,
}

impl TryFrom<Bytes> for ProxyMessage {
    type Error = ProtocolError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        bincode::deserialize(&value).map_err(ProtocolError::Deserialize)
    }
}

impl TryFrom<ProxyMessage> for Bytes {
    type Error = ProtocolError;

    fn try_from(value: ProxyMessage) -> Result<Self, Self::Error> {
        bincode::serialize(&value)
            .map(Bytes::from)
            .map_err(ProtocolError::Serialize)
    }
}
