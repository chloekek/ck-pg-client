use {crate::{Error, Result}, std::{io::{Read, Write}, slice}};

/// Implementation of the [_SSL Session Encryption_][spec] flow.
///
/// No data must be sent on the stream prior to calling this function.
///
/// If this function returns [`Ok`], the caller can initiate the SSL handshake.
/// If this function returns [`Err`], either an I/O error occurred,
/// the server denied the SSL request (meaning SSL is not available),
/// or the server responded with gibberish (which may be the result of
/// a person-in-the-middle attack, a malfunctioning connection,
/// or a version of PostgreSQL that predates SSL support).
///
#[doc = crate::pgdoc::ssl_session_encryption!("spec")]
pub fn ssl_session_encryption(stream: &mut (impl Read + Write)) -> Result<()>
{
    let ssl_request = [0, 0, 0, 8, 4, 210, 22, 47];
    stream.write_all(&ssl_request)?;

    let mut byte = 0;
    stream.read_exact(slice::from_mut(&mut byte))?;

    match byte {
        b'S' => Ok(()),
        b'N' => Err(Error::SslServerUnwilling),
        _    => Err(Error::SslRequestGibberish(byte)),
    }
}
