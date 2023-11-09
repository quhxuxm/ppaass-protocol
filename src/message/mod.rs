mod address;
mod agent;
mod encryption;
mod proxy;

pub use address::*;
pub use agent::*;
pub use encryption::*;
pub use proxy::*;

use bytes::Bytes;
use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};

use crate::error::ProtocolError;

/// The proxy message
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Constructor)]
pub struct WrapperMessage {
    /// The unique id to idenfiy this message
    pub unique_id: String,
    /// The user token
    pub user_token: String,
    /// The encryption of the payload used for this message
    pub encryption: Encryption,
    /// The message payload
    pub payload: Bytes,
}

impl TryFrom<Bytes> for WrapperMessage {
    type Error = ProtocolError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        bincode::deserialize(&value).map_err(ProtocolError::Deserialize)
    }
}

impl TryFrom<WrapperMessage> for Bytes {
    type Error = ProtocolError;

    fn try_from(value: WrapperMessage) -> Result<Self, Self::Error> {
        bincode::serialize(&value)
            .map(Bytes::from)
            .map_err(ProtocolError::Serialize)
    }
}
