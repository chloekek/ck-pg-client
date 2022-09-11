//! Connecting to PostgreSQL databases.

pub use self::{conventions::*, initiate_ssl::*, start_up::*};

use std::io;

mod conventions;
mod initiate_ssl;
mod start_up;

/// Initiate GSSAPI session encryption given a stream.
///
/// This function will perform the [_GSSAPI Session Encryption_][spec] flow.
/// No data must be sent on the stream prior to calling this function.
///
#[doc = crate::pgdoc::gssapi_session_encryption!("spec")]
pub fn initiate_gssapi<S>(stream: &mut S, todo: !) -> io::Result<()>
{
    let _ = stream;
    todo
}

/// Submit a cancel request given a stream.
///
/// This function will perform the [_Canceling Requests in Progress_][s] flow.
/// No data must be sent on the stream prior to calling this function.
///
#[doc = crate::pgdoc::canceling_requests_in_progress!("s")]
pub fn cancel_request<S>(stream: &mut S, todo: !) -> io::Result<()>
{
    let _ = stream;
    todo
}
