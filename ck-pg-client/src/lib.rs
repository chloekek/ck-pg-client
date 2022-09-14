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

pub mod capabilities;
pub mod connectivity;
pub mod protocol;

mod error;
mod tests;
mod usize_conversions;

pub struct ConnectionBuilder
{
    user: Option<Vec<u8>>,
    database: Option<Vec<u8>>,
    password: Option<Vec<u8>>,
}

impl ConnectionBuilder
{
    /// Create a builder with all settings missing.
    pub fn new() -> Self
    {
        Self{user: None, database: None, password: None}
    }

    /// Parse a libpq connection string.
    ///
    /// This replicates the behavior of the [libpq] library,
    /// including the use of the `PG*` environment variables.
    /// Existing connection builder settings are overwritten;
    /// set them after calling this method if you want them to take precedence.
    ///
    #[doc = crate::pgdoc::connection_strings!("libpq")]
    pub fn libpq_dsn(&mut self, _dsn: &str, todo: !) -> Result<Self>
    {
        todo
    }

    /// Connect as the specified database user.
    ///
    /// This parameter is required; if you do not set it, connecting will fail.
    pub fn user(&mut self, user: impl Into<Vec<u8>>) -> &mut Self
    {
        self.user = Some(user.into());
        self
    }

    /// Connect to the specified database.
    ///
    /// If not specified, the database named after the user is connected to.
    pub fn database(&mut self, database: impl Into<Vec<u8>>) -> &mut Self
    {
        self.database = Some(database.into());
        self
    }

    /// Authenticate using the specified password.
    ///
    /// If not specified, authentication will
    /// fail if the server requires a password.
    pub fn password(&mut self, password: impl Into<Vec<u8>>) -> &mut Self
    {
        self.password = Some(password.into());
        self
    }
}
