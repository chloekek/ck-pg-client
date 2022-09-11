//! Connecting to PostgreSQL databases.

pub use self::conventions::*;

use std::io;

mod conventions;

/// Initiate SSL session encryption given a socket.
///
/// The socket must already be connected to the database cluster.
/// This function will perform the [_SSL Session Encryption_][spec] flow.
/// No data must be sent on the socket prior to calling this function.
///
#[doc = crate::pgdoc::ssl_session_encryption!("spec")]
pub fn initiate_ssl<S>(socket: &mut S, todo: !) -> io::Result<()>
{
    let _ = socket;
    todo
}

/// Initiate GSSAPI session encryption given a socket.
///
/// The socket must already be connected to the database cluster.
/// This function will perform the [_GSSAPI Session Encryption_][spec] flow.
/// No data must be sent on the socket prior to calling this function.
///
#[doc = crate::pgdoc::gssapi_session_encryption!("spec")]
pub fn initiate_gssapi<S>(socket: &mut S, todo: !) -> io::Result<()>
{
    let _ = socket;
    todo
}

/// Initiate a database connection given a socket.
///
/// The socket must already be connected to the database cluster.
/// This function will perform the _Start-up_ flow.
/// No data must be sent on the socket prior to calling this function,
/// except for possible [SSL] or [GSSAPI] session encryption initiation
/// (in which case `S` must implement said encryption protocol).
///
/// [SSL]: `initiate_ssl`
/// [GSSAPI]: `initiate_gssapi`
pub fn start_up<S>(socket: &mut S) -> io::Result<()>
{
    todo!()
}
