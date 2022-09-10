//! PostgreSQL client library using synchronous I/O.

// This crate implements a network protocol and deals with untrusted data.
#![deny(unsafe_code)]

#![feature(cstr_from_bytes_until_nul)]
#![feature(write_all_vectored)]

#![cfg_attr(test, feature(exit_status_error))]

#![warn(missing_docs)]

pub mod connectivity;
pub mod protocol;

mod tests;
mod usize_conversions;