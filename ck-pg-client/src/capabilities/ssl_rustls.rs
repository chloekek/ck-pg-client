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

    fn handshake(&self, mut socket: Socket, server_name: &str)
        -> Result<Self::Stream>
    {
        let config = self.config.clone();
        let server_name = e(ServerName::try_from(server_name))?;
        let mut connection = e(ClientConnection::new(config, server_name))?;
        e(connection.complete_io(&mut socket))?;
        Ok(StreamOwned{conn: connection, sock: socket})
    }
}

fn e<R, E>(result: std::result::Result<R, E>) -> Result<R>
    where E: 'static + std::error::Error + Send + Sync
{
    result.map_err(|err| Error::SslHandshake(Box::new(err)))
}
