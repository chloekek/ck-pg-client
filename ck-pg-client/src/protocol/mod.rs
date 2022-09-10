//! Low-level implementation of the PostgreSQL protocol.
//!
//! The protocol is documentated in the PostgreSQL manual
//! under the chapter [_Frontend/Backend Protocol_][spec].
//! This module facilitates serialization of frontend messages,
//! and implements receiving and deserialization of backend messages.
//!
//! Optimal serialization of frontend messages depends on the interface
//! behind which it happens, so there is no uniform interface for doing that.
//!
//! [spec]: https://www.postgresql.org/docs/current/protocol.html

pub use self::{
    backend_message::*,
    read_backend_message::*,
};

mod backend_message;
mod read_backend_message;
