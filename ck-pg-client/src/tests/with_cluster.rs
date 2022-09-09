use {
    scopeguard::defer,
    std::{
        ffi::OsStr,
        fs::create_dir,
        path::Path,
        process::Command,
        thread::sleep,
        time::Duration,
    },
    tempfile::tempdir,
};

/// Spawn a temporary PostgreSQL cluster.
///
/// The cluster is created in a temporary directory and spawned.
/// It is killed and removed when `f` returns or panics.
/// `f` receives the path to the cluster's socket directory.
pub fn with_cluster<F, R>(f: F) -> R
    where F: FnOnce(&Path) -> R
{
    let test_dir = tempdir().unwrap();
    let cluster_dir = test_dir.path().join("cluster");
    let sockets_dir = test_dir.path().join("sockets");
    create_dir(&cluster_dir).unwrap();
    create_dir(&sockets_dir).unwrap();

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
    let mut postgres =
        new_robust_command("postgres")
        // Disable listening on TCP addresses to avoid conflicting binds.
        .arg("-c").arg("listen_addresses=")
        .arg("-D").arg(cluster_dir)
        .arg("-k").arg(&sockets_dir)
        .spawn().unwrap();

    // Kill the PostgreSQL server at the end of scope.
    defer! {
        let _ = postgres.kill();
        let _ = postgres.wait();
    }

    // Wait until the PostgreSQL cluster is ready.
    wait_until_pg_ready(&sockets_dir);

    f(&sockets_dir)
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
fn wait_until_pg_ready(sockets_dir: &Path)
{
    for _ in 0 .. 10 {
        let status =
            new_robust_command("pg_isready")
            .arg("--host").arg(sockets_dir)
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
