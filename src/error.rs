use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Fail to serialize message because of error: {0:?}")]
    Serialize(#[source] bincode::Error),
    #[error("Fail to deserialize message because of error: {0:?}")]
    Deserialize(#[source] bincode::Error),
    #[error("Protocol error happeb because of error: {0}")]
    Other(String),
}
