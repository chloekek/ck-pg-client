//! Connecting to PostgreSQL databases.

use std::{
    io::{self, IoSlice, IoSliceMut, Read, Write},
    net::TcpStream,
    path::{Path, PathBuf},
};

#[cfg(unix)]
use std::{fs::File, os::unix::{io::OwnedFd, net::UnixStream}};

/// The port on which PostgreSQL listens by default.
pub const DEFAULT_PORT: u16 = 5432;

/// The path to a PostgreSQL Unix socket given the socket directory and port.
///
/// Unix sockets don't actually have port numbers, but PostgreSQL
/// pretends they do by using [a certain convention][convention].
/// This function implements that convention.
///
#[doc = crate::pgdoc::guc_unix_socket_directories!("convention")]
pub fn unix_socket_path(sockets_dir: &Path, port: u16) -> PathBuf
{
    sockets_dir.join(format!(".s.PGSQL.{port}"))
}

/// TCP or Unix socket, or just a TCP socket on non-Unix platforms.
pub struct Socket(
    // On Unix, use File instead of a sum of TcpStream and UnixStream.
    // This is more efficient as we don't need the discriminant of an enum.
    // File, TcpStream, and UnixStream use the same system calls for I/O.
    #[cfg(unix)] File,
    #[cfg(not(unix))] TcpStream,
);

impl Socket
{
    pub fn from_tcp_stream(tcp: TcpStream) -> Self
    {
        #[cfg(unix)] {
            Self(File::from(OwnedFd::from(tcp)))
        }

        #[cfg(not(unix))] {
            Self(tcp)
        }
    }

    #[cfg(unix)]
    pub fn from_unix_stream(unix: UnixStream) -> Self
    {
        Self(File::from(OwnedFd::from(unix)))
    }
}

impl Read for Socket
{
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
    {
        self.0.read(buf)
    }

    #[inline(always)]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut]) -> io::Result<usize>
    {
        self.0.read_vectored(bufs)
    }
}

impl Write for Socket
{
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>
    {
        self.0.write(buf)
    }

    #[inline(always)]
    fn write_vectored(&mut self, bufs: &[IoSlice]) -> io::Result<usize>
    {
        self.0.write_vectored(bufs)
    }

    #[inline(always)]
    fn flush(&mut self) -> io::Result<()>
    {
        self.0.flush()
    }
}
