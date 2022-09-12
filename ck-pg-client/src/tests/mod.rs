#![cfg(test)]

use {
    crate::{
        capabilities::Md5Unavailable,
        connectivity::{InitiateSslError, initiate_ssl, start_up},
        protocol::Receiver,
    },
    self::with_cluster::{WithCluster, with_cluster},
    std::{
        assert_matches::assert_matches,
        collections::VecDeque,
        net::TcpStream,
    },
};

#[cfg(feature = "rustls")]
use crate::capabilities::{Ssl, SslRustls};

#[cfg(feature = "rustls")]
mod rustls_util;

mod with_cluster;

#[cfg(feature = "rustls")]
#[test]
fn initiate_ssl_success()
{
    let options = WithCluster{enable_ssl: true};
    with_cluster(options, |_sockets_dir, port| {

        let mut receiver = Receiver::new(|fields| println!("{fields:?}"));

        let mut socket = TcpStream::connect(("localhost", port)).unwrap();
        initiate_ssl(&mut socket).unwrap();

        let ssl = SslRustls{config: rustls_util::rustls_config()};
        let mut stream = ssl.handshake(socket, "localhost").unwrap();

        start_up(
            &mut receiver,
            &mut stream,
            [
                ("user", "postgres"),
                ("database", "postgres"),
            ],
            &Md5Unavailable,
        ).unwrap();

    });
}

#[test]
fn initiate_ssl_server_unwilling()
{
    let options = WithCluster{enable_ssl: false};
    with_cluster(options, |_sockets_dir, port| {
        let mut stream = TcpStream::connect(("localhost", port)).unwrap();
        let error = initiate_ssl(&mut stream).unwrap_err();
        assert_matches!(error, InitiateSslError::ServerUnwilling);
    });
}

#[test]
fn initiate_ssl_received_gibberish()
{
    let mut stream = VecDeque::from(*b"XYZ");
    let result = initiate_ssl(&mut stream).unwrap_err();
    assert_matches!(result, InitiateSslError::ReceivedGibberish(b'X'));
    assert_eq!(stream, b"YZ\0\0\0\x08\x04\xD2\x16\x2F");
}
