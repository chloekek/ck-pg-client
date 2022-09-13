use {
    crate::{Error, Result},
    super::Ssl,
    rustls::{ClientConfig, ClientConnection, ServerName, StreamOwned},
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

    fn handshake(&self, socket: Socket, server_name: &str)
        -> Result<Self::Stream>
    {
        let config = self.config.clone();
        let server_name = ServerName::try_from(server_name)
            .map_err(|err| Error::SslHandshake(Box::new(err)))?;
        let connection = ClientConnection::new(config, server_name)
            .map_err(|err| Error::SslHandshake(Box::new(err)))?;
        Ok(StreamOwned{conn: connection, sock: socket})
    }
}
