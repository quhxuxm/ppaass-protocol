use std::fmt::Debug;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("Protocol error happen because of error: {0:?}")]
pub struct ProtocolError(pub String);
