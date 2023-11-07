use bytes::Bytes;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Encryption {
    Plain,
    Aes(Bytes),
}
