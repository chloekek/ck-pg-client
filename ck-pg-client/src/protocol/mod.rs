//! Low-level implementation of the PostgreSQL protocol.
//!
//! The protocol is documentated in the PostgreSQL manual
//! under the chapter [_Frontend/Backend Protocol_][spec].
//! This module implements receiving and deserialization of backend messages,
//! as well as the various message flows documented in the PostgreSQL manual.
//!
//! Optimal serialization of frontend messages depends on the interface
//! behind which it happens, so there is no uniform interface for doing that.
//!
#![doc = crate::pgdoc::frontend_backend_protocol!("spec")]

pub use self::{
    backend_message::*,
    frontend_message::*,
    receiver::*,
    ssl_session_encryption::*,
    startup::*,
};

mod backend_message;
mod frontend_message;
mod receiver;
mod ssl_session_encryption;
mod startup;
