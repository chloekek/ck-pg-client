//! PostgreSQL client library using synchronous I/O.
//!
//! # Cargo features
//!
//! The following [Cargo features] can be specified:
//!
//! ## md5
//!
//! Use the **md5** crate for `md5` password authentication.
//! If you wish to use another MD5 implementation,
//! you can implement the [`Md5`] trait yourself.
//!
//! ## rustls
//!
//! Use the **rustls** crate for SSL session encryption.
//! If you wish to use another SSL implementation,
//! you can implement the [`Ssl`] trait yourself.
//!
//! # Unsupported protocol features
//!
//!  - Streaming replication protocol.
//!  - Kerberos V5 authentication.
//!  - GSSAPI session encryption.
//!
//! [`Md5`]: `capabilities::Md5`
//! [`Ssl`]: `capabilities::Ssl`
//! [Cargo features]: https://doc.rust-lang.org/cargo/reference/features.html

// This crate implements a network protocol and deals with untrusted data.
#![deny(unsafe_code)]

#![feature(cstr_from_bytes_until_nul)]
#![feature(never_type)]

#![cfg_attr(test, feature(assert_matches))]
#![cfg_attr(test, feature(exit_status_error))]

#![warn(missing_docs)]

#[macro_use] mod pgdoc;

pub use self::error::*;

use {
    crate::{connectivity::Socket, protocol::{ErrorNoticeFieldArray, Receiver}},
    std::io::{Read, Write},
};

pub mod capabilities;
pub mod connectivity;
pub mod protocol;

mod error;
mod tests;
mod usize_conversions;

/// Open connection to a database.
pub struct Connection
{
    /// Either [`Socket`] or [`Ssl::Stream`].
    transport: Box<dyn Transport>,

    receiver: Receiver,
}

trait Transport: Read + Write + Send { }
impl<T> Transport for T where T: Read + Write + Send { }

impl Connection
{
    pub fn connect(
        md5: &impl capabilities::Md5,
        ssl: &impl capabilities::Ssl,
        on_notice: impl 'static + FnMut(ErrorNoticeFieldArray) + Send,
        options: &ConnectionOptions,
    ) -> Result<Self>
    {
        let transport: Socket = todo!();

        let transport: Box<dyn Transport> =
            match options.sslmode {
                Sslmode::Disable =>
                    Box::new(transport),
                Sslmode::Require => {
                    protocol::ssl_session_encryption(&mut transport)?;
                    let ssl_stream = ssl.handshake(transport, todo!())?;
                    Box::new(ssl_stream)
                }
            };

        let mut receiver = Receiver::new(on_notice);

        protocol::startup(
            md5,
            &mut receiver,
            &mut transport,
            &options.user,
            &options.dbname,
        )?;

        Ok(Self{transport, receiver})
    }
}

/// Options describing a database connection.
pub struct ConnectionOptions
{
    /// The database name.
    ///
    /// Technically, this is the `database` parameter
    /// included in the `StartupMessage` message.
    pub dbname: Vec<u8>,

    /// PostgreSQL user name to connect as.
    ///
    /// Technically, this is the `user` parameter
    /// included in the `StartupMessage` message.
    pub user: Vec<u8>,

    /// Password to be used if the server demands password authentication.
    pub password: Option<Vec<u8>>,

    /// Whether to use plaintext or SSL encrypted communication.
    ///
    /// This fulfills the same purpose as the [`sslmode`] parameter in libpq,
    /// but is more direct: you must specify whether you want SSL or not.
    /// [`a_la_libpq`] only accepts `disable` and `require` for [`sslmode`];
    /// other values produce an error so the caller must disambiguate them.
    /// The certificate verification requirements are up to [`Ssl::handshake`].
    ///
    /// [`a_la_libpq`]: `Self::a_la_libpq`
    /// [`Ssl::handshake`]: `capabilities::Ssl::handshake`
    #[doc = crate::pgdoc::sslmode!("`sslmode`")]
    pub sslmode: Sslmode,
}

pub enum Sslmode
{
    Disable,
    Require,
}

impl ConnectionOptions
{
    /// Parse a libpq connection string.
    ///
    /// This replicates the behavior of the [libpq] library,
    /// including the use of the `PG*` environment variables.
    ///
    #[doc = crate::pgdoc::connection_strings!("libpq")]
    pub fn a_la_libpq(_connection_string: &str, todo: !) -> Result<Self>
    {
        todo
    }
}
