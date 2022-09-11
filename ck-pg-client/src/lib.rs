//! PostgreSQL client library using synchronous I/O.

// This crate implements a network protocol and deals with untrusted data.
#![deny(unsafe_code)]

#![feature(cstr_from_bytes_until_nul)]
#![feature(never_type)]

#![cfg_attr(test, feature(exit_status_error))]

#![warn(missing_docs)]

#[macro_use] mod pgdoc;

pub mod connectivity;
pub mod protocol;

mod tests;
mod usize_conversions;
