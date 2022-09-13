use {crate::{Error, Result}, super::Ssl, thiserror::Error};

/// Implementation of the [`Ssl`] trait that unconditionally fails to handshake.
#[derive(Debug, Error)]
#[error("SslUnavailable was used")]
pub struct SslUnavailable;

impl<Socket> Ssl<Socket> for SslUnavailable
{
    type Stream = !;

    fn handshake(&self, _socket: Socket, _server_name: &str)
        -> Result<Self::Stream>
    {
        Err(Error::SslHandshake(Box::new(SslUnavailable)))
    }
}
