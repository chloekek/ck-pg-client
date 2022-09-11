#![cfg(test)]

use {
    crate::connectivity::{InitiateSslError, initiate_ssl, start_up},
    self::with_cluster::{WithCluster, with_cluster},
    std::{
        assert_matches::assert_matches,
        collections::VecDeque,
        net::TcpStream,
    },
};

mod rustls_util;
mod with_cluster;

#[test]
fn initiate_ssl_success()
{
    let options = WithCluster{enable_ssl: true};
    with_cluster(options, |_sockets_dir, port| {

        let mut stream = TcpStream::connect(("localhost", port)).unwrap();
        initiate_ssl(&mut stream).unwrap();

        let mut rustls = rustls_util::create_client_connection();
        let mut stream = rustls::Stream::new(&mut rustls, &mut stream);

        start_up(&mut stream).unwrap();

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
