use super::Ssl;

/// Implementation of the [`Ssl`] trait that unconditionally fails to handshake.
pub struct SslUnavailable;

impl<Socket> Ssl<Socket> for SslUnavailable
{
    type Stream = !;

    type Error = SslUnavailable;

    fn handshake(&self, socket: Socket, server_name: &str)
        -> Result<Self::Stream, Self::Error>
    {
        let _ = socket;
        let _ = server_name;
        Err(SslUnavailable)
    }
}
