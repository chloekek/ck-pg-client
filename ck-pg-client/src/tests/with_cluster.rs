use {
    scopeguard::defer,
    std::{
        env::current_dir,
        ffi::{OsStr, OsString},
        fs::create_dir,
        path::Path,
        process::Command,
        sync::atomic::{AtomicU16, Ordering::SeqCst},
        thread::sleep,
        time::Duration,
    },
    tempfile::tempdir,
};

/// Concatenate strings (potentially of different types) into an [`OsString`].
macro_rules! osstring
{
    [$($str:expr),* $(,)?] => {
        OsString::from_iter([$(OsStr::new($str)),*])
    };
}

/// The PostgreSQL port number to use by the next call to [`with_cluster`].
static NEXT_PORT: AtomicU16 = AtomicU16::new(7000);

/// Options for [`with_cluster`].
pub struct WithCluster
{
    /// Whether to enable SSL in PostgreSQL.
    pub enable_ssl: bool,
}

/// Spawn a temporary PostgreSQL cluster.
///
/// The cluster is created in a temporary directory and spawned.
/// It is killed and removed when `f` returns or panics.
/// `f` receives the path to the cluster's socket directory
/// and the port on which the cluster listens for connections.
/// The cluster listens for both TCP and Unix connections.
pub fn with_cluster<F, R>(options: WithCluster, f: F) -> R
    where F: FnOnce(&Path, u16) -> R
{
    let current_dir = current_dir().unwrap();
    let test_dir = tempdir().unwrap();
    let cluster_dir = test_dir.path().join("cluster");
    let sockets_dir = test_dir.path().join("sockets");
    create_dir(&cluster_dir).unwrap();
    create_dir(&sockets_dir).unwrap();

    // Select port number for PostgreSQL to listen on.
    let port = NEXT_PORT.fetch_add(1, SeqCst);

    // Create PostgreSQL cluster.
    new_robust_command("initdb")
        .arg("--pgdata").arg(&cluster_dir)
        .arg("--encoding").arg("UTF-8")
        .arg("--locale").arg("C")
        .arg("--pwfile").arg("testdata/pwfile")
        .arg("--username").arg("postgres")
        .status().unwrap()
        .exit_ok().unwrap();

    // Spawn PostgreSQL database server.
    let mut postgres = new_robust_command("postgres");
    macro_rules! config { ($($arg:expr),*) => {
        postgres.arg("-c").arg(osstring!($($arg),*)) }; }
    config!("port=", &port.to_string());
    if options.enable_ssl {
        config!("ssl=on");
        config!("ssl_cert_file=", &current_dir, "/testdata/server.crt");
        config!("ssl_key_file=", &current_dir, "/testdata/server.key");
    }
    postgres.arg("-D").arg(cluster_dir);
    postgres.arg("-k").arg(&sockets_dir);
    let mut postgres = postgres.spawn().unwrap();

    // Kill the PostgreSQL server at the end of scope.
    defer! {
        let _ = postgres.kill();
        let _ = postgres.wait();
    }

    // Wait until the PostgreSQL cluster is ready.
    wait_until_pg_ready(&sockets_dir, port);

    f(&sockets_dir, port)
}

/// Create a command which is killed when the test program terminates.
///
/// This prevents processes from lingering, requiring manual cleanup.
/// This works by setting the parent death signal to SIGKILL.
fn new_robust_command<S>(program: S) -> Command
    where S: AsRef<OsStr>
{
    if cfg!(target_os = "linux") {
        let mut command = Command::new("setpriv");
        command.arg("--pdeathsig").arg("KILL");
        command.arg("--").arg(program);
        command
    } else {
        // pdeathsig is a Linux-specific feature.
        // On other platforms, hope for the best.
        Command::new(program)
    }
}

/// Wait until PostgreSQL is ready.
///
/// If this takes too long then we fail the test.
/// That likely means that PostgreSQL did not start.
fn wait_until_pg_ready(sockets_dir: &Path, port: u16)
{
    for _ in 0 .. 10 {
        let status =
            new_robust_command("pg_isready")
            .arg("--host").arg(sockets_dir)
            .arg("--port").arg(port.to_string())
            .arg("--username").arg("postgres")
            .status().unwrap();
        if status.success() {
            return;
        } else {
            sleep(Duration::from_millis(100));
        }
    }
    panic!("PostgreSQL did not become ready");
}
