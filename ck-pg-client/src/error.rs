use {std::{error, io}, thiserror::Error};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error
{
    #[error("I/O: {0}")]
    Io(#[from] io::Error),

    #[error("frontend message: string field contains nul")]
    Nul,

    #[error("backend message: invalid syntax")]
    BackendMessageParse,

    #[error("backend message: unexpected identifier")]
    BackendMessageUnexpected,

    #[error("SSL: server is unwilling to encrypt communications")]
    SslServerUnwilling,

    #[error("SSL: received gibberish in response to SSL request: {0:x}")]
    SslRequestGibberish(u8),

    #[error("SSL: handshake: {0}")]
    SslHandshake(Box<dyn error::Error + Send + Sync>),
}
