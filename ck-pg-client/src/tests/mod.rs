#![cfg(test)]

use {
    crate::{
        ConnectionOptions,
        Error,
        PgClient,
        Sslmode,
        capabilities::{Md5Unavailable, SslUnavailable},
        protocol::ssl_session_encryption,
    },
    self::with_cluster::{WithCluster, with_cluster},
    std::{assert_matches::assert_matches, collections::VecDeque},
};

#[cfg(feature = "rustls")]
use crate::capabilities::SslRustls;

#[cfg(feature = "rustls")]
mod rustls_util;

mod with_cluster;

#[cfg(feature = "rustls")]
#[test]
fn connect_ssl_required_success()
{
    let options = WithCluster{enable_ssl: true};
    with_cluster(options, |_sockets_dir, port| {

        let options = ConnectionOptions{
            host: "localhost".into(),
            port,
            dbname: "postgres".into(),
            user: "postgres".into(),
            password: None,
            sslmode: Sslmode::Require,
        };

        PgClient::connect(
            &Md5Unavailable,
            &SslRustls{config: rustls_util::rustls_config()},
            |notice| println!("{notice:?}"),
            &options,
        ).unwrap();

    });
}

#[test]
fn connect_ssl_required_server_unwilling()
{
    let options = WithCluster{enable_ssl: false};
    with_cluster(options, |_sockets_dir, port| {

        let options = ConnectionOptions{
            host: "localhost".into(),
            port,
            dbname: "postgres".into(),
            user: "postgres".into(),
            password: None,
            sslmode: Sslmode::Require,
        };

        let error = PgClient::connect(
            &Md5Unavailable,
            &SslUnavailable,
            |notice| println!("{notice:?}"),
            &options,
        ).map(|_| ()).unwrap_err();

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
