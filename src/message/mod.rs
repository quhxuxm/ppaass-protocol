pub mod values;
use crate::error::ProtocolError;
use crate::message::values::address::UnifiedAddress;
use bytes::Bytes;
use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};
/// The encryption of the packet payload
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PpaassPacketPayloadEncryption {
    /// The packet payload is not encrypted
    Plain,
    /// The packet payload is encrypted with AES,
    /// the content of the variant is the encryption
    /// used for AES.
    Aes(Bytes),
}

impl TryFrom<Bytes> for PpaassPacketPayloadEncryption {
    type Error = ProtocolError;
    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        let result = bincode::deserialize::<PpaassPacketPayloadEncryption>(&value)?;
        Ok(result)
    }
}

impl TryFrom<PpaassPacketPayloadEncryption> for Bytes {
    type Error = ProtocolError;
    fn try_from(value: PpaassPacketPayloadEncryption) -> Result<Self, Self::Error> {
        let result = bincode::serialize(&value)?;
        Ok(result.into())
    }
}

/// The payload of the packet
#[derive(Serialize, Deserialize, Debug)]
pub enum PpaassPacketPayload {
    /// The payload is used for encryption key exchange,
    /// the content of the encryption is the encryption key,
    /// the encryption key is encrypted with RSA
    KeyExchange,
    /// The payload is used for transfer data.
    Content {
        /// The source address of the payload
        src_address: UnifiedAddress,
        /// The destination address of the payload
        dest_address: UnifiedAddress,
        /// The content of the payload
        data: Bytes,
    },
}

impl TryFrom<Bytes> for PpaassPacketPayload {
    type Error = ProtocolError;
    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        let result = bincode::deserialize::<PpaassPacketPayload>(&value)?;
        Ok(result)
    }
}

impl TryFrom<PpaassPacketPayload> for Bytes {
    type Error = ProtocolError;
    fn try_from(value: PpaassPacketPayload) -> Result<Self, Self::Error> {
        let result = bincode::serialize(&value)?;
        Ok(result.into())
    }
}

#[derive(Serialize, Deserialize, Debug, Constructor)]
pub struct PpaassPacket {
    packet_id: String,
    user_token: String,
    encryption: PpaassPacketPayloadEncryption,
    payload: Bytes,
}
impl PpaassPacket {
    pub fn packet_id(&self) -> &str {
        &self.packet_id
    }

    pub fn user_token(&self) -> &str {
        &self.user_token
    }

    pub fn encryption(&self) -> &PpaassPacketPayloadEncryption {
        &self.encryption
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

impl TryFrom<Bytes> for PpaassPacket {
    type Error = ProtocolError;
    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        let result = bincode::deserialize::<PpaassPacket>(&value)?;
        Ok(result)
    }
}

impl TryFrom<PpaassPacket> for Bytes {
    type Error = ProtocolError;
    fn try_from(value: PpaassPacket) -> Result<Self, Self::Error> {
        let result = bincode::serialize(&value)?;
        Ok(result.into())
    }
}
