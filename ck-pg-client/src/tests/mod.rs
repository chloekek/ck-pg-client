#![cfg(test)]

use {
    crate::{Error, protocol::{Receiver, ssl_session_encryption, startup}},
    self::with_cluster::{WithCluster, with_cluster},
    std::{
        assert_matches::assert_matches,
        collections::VecDeque,
        net::TcpStream,
    },
};

#[cfg(feature = "rustls")]
use crate::capabilities::{Md5Unavailable, Ssl, SslRustls};

#[cfg(feature = "rustls")]
mod rustls_util;

mod with_cluster;

#[cfg(feature = "rustls")]
#[test]
fn ssl_session_encryption_success()
{
    let options = WithCluster{enable_ssl: true};
    with_cluster(options, |_sockets_dir, port| {

        let mut receiver = Receiver::new(|fields| println!("{fields:?}"));

        let mut socket = TcpStream::connect(("localhost", port)).unwrap();
        ssl_session_encryption(&mut socket).unwrap();

        let ssl = SslRustls{config: rustls_util::rustls_config()};
        let mut stream = ssl.handshake(socket, "localhost").unwrap();

        startup(
            &Md5Unavailable,
            &mut receiver,
            &mut stream,
            [
                ("user", "postgres"),
                ("database", "postgres"),
            ],
        ).unwrap();

    });
}

#[test]
fn ssl_session_encryption_server_unwilling()
{
    let options = WithCluster{enable_ssl: false};
    with_cluster(options, |_sockets_dir, port| {
        let mut stream = TcpStream::connect(("localhost", port)).unwrap();
        let error = ssl_session_encryption(&mut stream).unwrap_err();
        assert_matches!(error, Error::SslServerUnwilling);
    });
}

#[test]
fn ssl_session_encryption_received_gibberish()
{
    let mut stream = VecDeque::from(*b"XYZ");
    let result = ssl_session_encryption(&mut stream).unwrap_err();
    assert_matches!(result, Error::SslRequestGibberish(b'X'));
    assert_eq!(stream, b"YZ\0\0\0\x08\x04\xD2\x16\x2F");
}
