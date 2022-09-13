pub use self::{
    md5_md5::*,
    md5_unavailable::*,
    ssl_rustls::*,
    ssl_unavailable::*,
};

use crate::Result;

#[cfg(feature = "md5")]
mod md5_md5;
#[cfg(not(feature = "md5"))]
mod md5_md5 { }

mod md5_unavailable;

#[cfg(feature = "rustls")]
mod ssl_rustls;
#[cfg(not(feature = "rustls"))]
mod ssl_rustls { }

mod ssl_unavailable;

pub trait Md5
{
    /// Hash a given plaintext using MD5.
    ///
    /// MD5 is used for [`md5` password authentication][spec].
    /// Returning [`None`] indicates a lack of support for MD5,
    /// in which case `md5` password authentication fails immediately.
    ///
    #[doc = crate::pgdoc::password_authentication!("spec")]
    fn md5(&self, plaintext: &[u8]) -> Option<[u8; 16]>;
}

pub trait Ssl<Socket>
{
    type Stream;

    fn handshake(&self, socket: Socket, server_name: &str)
        -> Result<Self::Stream>;
}
