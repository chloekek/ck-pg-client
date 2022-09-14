use {
    crate::{Error, Result, connectivity::Socket},
    super::Ssl,
    std::io::{self, Read, Write},
    thiserror::Error,
};

/// Implementation of the [`Ssl`] trait that unconditionally fails to handshake.
#[derive(Debug, Error)]
#[error("SslUnavailable was used")]
pub struct SslUnavailable;

/// Uninhabited type that implements [`Read`] and [`Write`].
///
/// Used as the SSL stream type for [`SslUnavailable`].
pub enum SslUnavailableStream
{
}

impl Ssl for SslUnavailable
{
    type Stream = SslUnavailableStream;

    fn handshake(&self, _socket: Socket, _server_name: &str)
        -> Result<Self::Stream>
    {
        Err(Error::SslHandshake(Box::new(SslUnavailable)))
    }
}

impl Read for SslUnavailableStream
{
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize>
    {
        match *self { }
    }
}

impl Write for SslUnavailableStream
{
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize>
    {
        match *self { }
    }

    fn flush(&mut self) -> io::Result<()>
    {
        match *self { }
    }
}
