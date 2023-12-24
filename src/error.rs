use thiserror::Error;
#[derive(Error, Debug)]

pub enum ProtocolError {
    #[error("Protocol error happen because of standard io: {_0:?}")]
    StdIo(#[from] std::io::Error),
    #[error("Protocol error happen because of reason: {_0:?}")]
    Other(String),
}
