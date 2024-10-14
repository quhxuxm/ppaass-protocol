pub mod values;
use crate::error::ProtocolError;
use crate::message::values::address::UnifiedAddress;
use bytes::Bytes;
use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Encryption {
    Plain,
    Aes(Bytes),
}

impl TryFrom<Bytes> for Encryption {
    type Error = ProtocolError;
    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        let result = bincode::deserialize::<Encryption>(&value)?;
        Ok(result)
    }
}

impl TryFrom<Encryption> for Bytes {
    type Error = ProtocolError;
    fn try_from(value: Encryption) -> Result<Self, Self::Error> {
        let result = bincode::serialize(&value)?;
        Ok(result.into())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Payload {
    KeyExchange,
    Content {
        src_address: UnifiedAddress,
        dest_address: UnifiedAddress,
        data: Option<Bytes>,
    },
}

impl TryFrom<Bytes> for Payload {
    type Error = ProtocolError;
    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        let result = bincode::deserialize::<Payload>(&value)?;
        Ok(result)
    }
}

impl TryFrom<Payload> for Bytes {
    type Error = ProtocolError;
    fn try_from(value: Payload) -> Result<Self, Self::Error> {
        let result = bincode::serialize(&value)?;
        Ok(result.into())
    }
}

#[derive(Serialize, Deserialize, Debug, Constructor)]
pub struct Packet {
    packet_id: String,
    user_token: String,
    encryption: Encryption,
    payload: Bytes,
}
impl Packet {
    pub fn packet_id(&self) -> &str {
        &self.packet_id
    }

    pub fn user_token(&self) -> &str {
        &self.user_token
    }

    pub fn encryption(&self) -> &Encryption {
        &self.encryption
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

impl TryFrom<Bytes> for Packet {
    type Error = ProtocolError;
    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        let result = bincode::deserialize::<Packet>(&value)?;
        Ok(result)
    }
}

impl TryFrom<Packet> for Bytes {
    type Error = ProtocolError;
    fn try_from(value: Packet) -> Result<Self, Self::Error> {
        let result = bincode::serialize(&value)?;
        Ok(result.into())
    }
}
