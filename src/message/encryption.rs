use bytes::Bytes;
use serde_derive::{Deserialize, Serialize};

/// The encryption of the message payload
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Encryption {
    /// The plain value
    Plain,
    /// The aes encrypted value, inner value is the aes encryption key
    Aes(Bytes),
}
