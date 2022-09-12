//! Connecting to PostgreSQL databases.

use std::path::{Path, PathBuf};

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
