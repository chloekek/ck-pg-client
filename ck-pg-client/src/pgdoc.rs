//! Macros for linking to PostgreSQL documentation sections.
//!
//! In the PostgreSQL documentation, hyperlinks to sections may include
//! section numbers which are unstable across PostgreSQL versions.
//! We aggregate all hyperlinks into this module so it is easy
//! to update all the links when we bump the PostgreSQL version number.

/// Define a macro that takes a Markdown hyperlink label
/// and expands into a rustdoc snippet that defines said hyperlink.
macro_rules! pgdoc
{
    ($name:ident, $uri:literal) => {
        macro_rules! $name {
            ($label:literal) => {
                concat!("[", $label, "]: https://www.postgresql.org/docs/14", $uri)
            };
        }
        pub (crate) use $name;
    };
}

pgdoc!(connection_strings, "/libpq-connect.html#LIBPQ-CONNSTRING");
pgdoc!(frontend_backend_protocol, "/protocol.html");
pgdoc!(guc_unix_socket_directories, "/runtime-config-connection.html#GUC-UNIX-SOCKET-DIRECTORIES");
pgdoc!(password_authentication, "/auth-password.html");
pgdoc!(ssl_session_encryption, "/protocol-flow.html#id-1.10.5.7.12");
pgdoc!(sslmode, "/libpq-connect.html#LIBPQ-CONNECT-SSLMODE");
pgdoc!(startup, "/protocol-flow.html#id-1.10.5.7.3");
