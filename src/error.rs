use config::ConfigError;
use thiserror::Error;

use crate::{protocol::error::ProtoError, server::router::RouterError};

#[derive(Error, Debug)]
pub enum HopperError {
    #[error("protocol error: {0}")]
    Protocol(#[from] ProtoError),

    #[error("routing error: {0}")]
    Router(#[from] RouterError),

    #[error("one of the two parties terminated the connection: {0}")]
    Disconnected(std::io::Error),

    #[error("configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("cannot listen on the specified ip: {0}")]
    Bind(std::io::Error),
}
