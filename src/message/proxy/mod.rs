mod payload;

use bytes::Bytes;
use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};

use crate::{error::ProtocolError, Encryption};

use self::payload::ProxyMessagePayload;

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Constructor)]
pub struct ProxyMessage {
    pub uniqueue_hash: Bytes,
    pub user_token: Bytes,
    pub encryption: Encryption,
    pub payload: ProxyMessagePayload,
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
