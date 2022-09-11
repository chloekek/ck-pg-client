use {std::{io::{self, Read, Write}, slice}, thiserror::Error};

/// Error returned by [`initiate_ssl`].
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum InitiateSslError
{
    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("server is unwilling to encrypt communications using SSL")]
    ServerUnwilling,

    #[error("received gibberish in response to SSL request: byte {0:x}")]
    ReceivedGibberish(u8),
}

/// Initiate SSL session encryption given a stream.
///
/// This function will perform the [_SSL Session Encryption_][spec] flow.
/// No data must be sent on the stream prior to calling this function.
///
/// If this function returns [`Ok`], the caller can initiate the SSL handshake.
/// After a successful SSL handshake, the caller should call [`start_up`]
/// or [`cancel_request`] using a stream wrapper
/// that transparently implements SSL.
///
/// If this function returns [`Err`], either an I/O error occurred,
/// the server denied the SSL request (meaning SSL is not available),
/// or the server responded with gibberish (which may be the result of
/// a person-in-the-middle attack, a malfunctioning connection,
/// or a version of PostgreSQL that predates SSL support).
///
/// [`start_up`]: `super::start_up`
/// [`cancel_request`]: `super::cancel_request`
#[doc = crate::pgdoc::ssl_session_encryption!("spec")]
pub fn initiate_ssl<S>(stream: &mut S) -> Result<(), InitiateSslError>
    where S: Read + Write
{
    let ssl_request = [0, 0, 0, 8, 4, 210, 22, 47];
    stream.write_all(&ssl_request)?;

    let mut byte = 0;
    stream.read_exact(slice::from_mut(&mut byte))?;

    match byte {
        b'S' => Ok(()),
        b'N' => Err(InitiateSslError::ServerUnwilling),
        _    => Err(InitiateSslError::ReceivedGibberish(byte)),
    }
}
