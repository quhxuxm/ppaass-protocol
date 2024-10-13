use thiserror::Error;
#[derive(Error, Debug)]

pub enum ProtocolError {
    #[error("IO error: {_0:?}")]
    Io(#[from] std::io::Error),
    #[error("Bincode error: {_0:?}")]
    Bincode(#[from] bincode::Error),
    #[error("Unknown error: {_0}")]
    Unknown(String),
}
