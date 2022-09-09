#![cfg(test)]

use {
    crate::{
        connectivity::{DEFAULT_PORT, unix_socket_path},
        protocol::read_backend_message,
    },
    self::with_cluster::*,
    std::{io::Write, os::unix::net::UnixStream},
};

mod with_cluster;

#[test]
fn example()
{
    with_cluster(|sockets_dir| {

        let socket_path = unix_socket_path(sockets_dir, DEFAULT_PORT);
        let mut socket = UnixStream::connect(socket_path).unwrap();

        let message = [
            0, 0, 0, 23,  // Length.
            0, 3, 0, 0,   // Protocol version.
            b'u', b's', b'e', b'r', 0,
            b'p', b'o', b's', b't', b'g', b'r', b'e', b's', 0,
            0,
        ];
        socket.write_all(&message).unwrap();

        let mut message = Vec::new();
        read_backend_message(&mut socket, &mut message).unwrap();
        println!("{message:?}");

        panic!();

    });
}
