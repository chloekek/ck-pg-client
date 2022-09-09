//! Low-level implementation of the PostgreSQL protocol.
//!
//! The protocol is documentated in the PostgreSQL manual
//! under the chapter [_Frontend/Backend Protocol_][spec].
//! This module implements serialization of frontend messages,
//! and receiving and deserialization of backend messages.
//!
//! [spec]: https://www.postgresql.org/docs/current/protocol.html

pub use self::read_backend_message::*;

mod read_backend_message;
