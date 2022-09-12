use {
    super::Ssl,
    rustls::{ClientConfig, ClientConnection, Error, ServerName, StreamOwned},
    std::{io::{Read, Write}, sync::Arc},
};

/// Implementation of the [`Ssl`] trait using the [`rustls`] crate.
pub struct SslRustls
{
    pub config: Arc<ClientConfig>,
}

impl<Socket> Ssl<Socket> for SslRustls
    where Socket: Read + Write
{
    type Stream = StreamOwned<ClientConnection, Socket>;

    type Error = Error;

    fn handshake(&self, socket: Socket, server_name: &str)
        -> Result<Self::Stream, Self::Error>
    {
        let config = self.config.clone();
        let server_name = ServerName::try_from(server_name)
            .map_err(|err| Error::General(err.to_string()))?;
        let connection = ClientConnection::new(config, server_name)?;
        Ok(StreamOwned{conn: connection, sock: socket})
    }
}
