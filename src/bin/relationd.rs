use anyhow::Result;
use anyhow::{Context, bail};
use interprocess::local_socket::{
    ListenerOptions,
    tokio::{Stream, prelude::*},
};
use relation::{socket_name, socket_path};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    env, fs,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    sync::mpsc,
    signal,
};

use relation::bridge;
use relation::{Command as ClientCommand, Request, Response};

const DETACHED_ENV: &str = "RELATION_DETACHED";
const FOREGROUND_FLAG: &str = "--foreground";

static RUNNING: AtomicBool = AtomicBool::new(false);
static SYSPROXY: AtomicBool = AtomicBool::new(false);

#[tokio::main]
async fn main() -> Result<()> {
    if should_detach() {
        #[cfg(unix)]
        ensure_daemon_not_running()?;

        let child = spawn_detached_child()?;
        println!("daemon started in background (pid: {})", child.id());
        return Ok(());
    }

    #[cfg(unix)]
    let _socket_guard = prepare_socket_file()?;

    let listener = ListenerOptions::new().name(socket_name()?).create_tokio()?;
    let (shutdown_tx, mut shutdown_rx) = mpsc::unbounded_channel();

    eprintln!("daemon listening");

    loop {
        tokio::select! {
            accept_result = listener.accept() => {
                let stream = accept_result?;
                let shutdown_tx = shutdown_tx.clone();
                tokio::spawn(async move {
                    if let Err(error) = handle_client(stream, shutdown_tx).await {
                        eprintln!("client error: {error}");
                    }
                });
            }
            Some(()) = shutdown_rx.recv() => {
                eprintln!("daemon shutting down");
                break;
            }
            signal_result = signal::ctrl_c() => {
                signal_result?;
                println!("daemon shutting down");
                break;
            }
        }
    }

    Ok(())
}

fn should_detach() -> bool {
    env::var_os(DETACHED_ENV).is_none() && !env::args().skip(1).any(|arg| arg == FOREGROUND_FLAG)
}

fn spawn_detached_child() -> Result<std::process::Child> {
    let args = env::args_os().skip(1).filter(|arg| arg != FOREGROUND_FLAG);
    let mut command = Command::new(env::current_exe()?);
    command
        .args(args)
        .env(DETACHED_ENV, "1")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    configure_detached_process(&mut command);

    let mut child = command.spawn()?;
    wait_for_child_startup(&mut child)?;
    Ok(child)
}

fn wait_for_child_startup(child: &mut std::process::Child) -> Result<()> {
    let deadline = Instant::now() + Duration::from_secs(2);

    loop {
        if let Some(status) = child.try_wait()? {
            bail!("daemon exited during startup with status {status}");
        }

        #[cfg(unix)]
        if std::os::unix::net::UnixStream::connect(socket_path()).is_ok() {
            return Ok(());
        }

        if Instant::now() >= deadline {
            bail!("daemon did not become ready before startup timeout");
        }

        thread::sleep(Duration::from_millis(25));
    }
}

#[cfg(unix)]
fn configure_detached_process(command: &mut Command) {
    use std::io;
    use std::os::unix::process::CommandExt;

    unsafe extern "C" {
        fn setsid() -> i32;
    }

    unsafe {
        command.pre_exec(|| {
            if setsid() == -1 {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        });
    }
}

#[cfg(windows)]
fn configure_detached_process(command: &mut Command) {
    use std::os::windows::process::CommandExt;

    const DETACHED_PROCESS: u32 = 0x0000_0008;
    const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    command.creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP | CREATE_NO_WINDOW);
}

#[cfg(unix)]
fn prepare_socket_file() -> Result<SocketGuard> {
    let path = socket_path();

    if daemon_is_running()? {
        bail!("daemon is already running: {path}");
    }

    if fs::metadata(path).is_ok() {
        fs::remove_file(path).with_context(|| format!("failed to remove stale socket {path}"))?;
    }

    Ok(SocketGuard { path })
}

#[cfg(unix)]
struct SocketGuard {
    path: &'static str,
}

#[cfg(unix)]
impl Drop for SocketGuard {
    fn drop(&mut self) {
        let _ = fs::remove_file(self.path);
    }
}

#[cfg(unix)]
fn ensure_daemon_not_running() -> Result<()> {
    let path = socket_path();
    if daemon_is_running()? {
        bail!("daemon is already running: {path}");
    }
    Ok(())
}

#[cfg(unix)]
fn daemon_is_running() -> Result<bool> {
    let path = socket_path();

    match std::os::unix::net::UnixStream::connect(path) {
        Ok(_) => Ok(true),
        Err(error)
            if matches!(
                error.kind(),
                std::io::ErrorKind::ConnectionRefused
                    | std::io::ErrorKind::NotFound
                    | std::io::ErrorKind::AddrNotAvailable
            ) =>
        {
            Ok(false)
        }
        Err(error) => Err(error).with_context(|| format!("failed to probe socket at {path}")),
    }
}

async fn handle_client(stream: Stream, shutdown_tx: mpsc::UnboundedSender<()>) -> Result<()> {
    let mut reader = BufReader::new(&stream);
    let mut writer = &stream;
    let mut line = String::new();
    let mut quit_flag = false;

    loop {
        line.clear();
        let read = reader.read_line(&mut line).await?;
        if read == 0 {
            break;
        }

        let request: Request = serde_json::from_str(line.trim())?;
        let response = match request.command {
            ClientCommand::Status => {
                if RUNNING.load(Ordering::Relaxed) {
                    Response::Running
                } else {
                    Response::Stopped
                }
            }
            ClientCommand::Start(config_path) => match bridge::start_safe(&config_path, 0) {
                Some(error) => Response::Error(error),
                None => {
                    RUNNING.store(true, Ordering::Relaxed);
                    Response::Ok
                }
            },
            ClientCommand::Stop => match bridge::stop_safe() {
                Some(error) => Response::Error(error),
                None => {
                    RUNNING.store(false, Ordering::Relaxed);
                    Response::Ok
                }
            },
            ClientCommand::EnableSysProxy((host, port, support_socks)) => {
                match bridge::enable_system_proxy_safe(&host, port as i64, support_socks) {
                    Some(error) => Response::Error(error),
                    None => {
                        SYSPROXY.store(true, Ordering::Relaxed);
                        Response::Ok
                    }
                }
            }
            ClientCommand::DisableSysProxy => match bridge::disable_system_proxy_safe() {
                Some(error) => Response::Error(error),
                None => {
                    SYSPROXY.store(false, Ordering::Relaxed);
                    Response::Ok
                }
            },
            ClientCommand::Quit => {
                quit_flag = true;
                if !RUNNING.load(Ordering::Relaxed) {
                    Response::Ok
                } else if SYSPROXY.load(Ordering::Relaxed) {
                    if let Some(error) = bridge::disable_system_proxy_safe() {
                        Response::Error(error)
                    } else {
                        SYSPROXY.store(false, Ordering::Relaxed);

                        if let Some(error) = bridge::stop_safe() {
                            Response::Error(error)
                        } else {
                            RUNNING.store(false, Ordering::Relaxed);
                            Response::Ok
                        }
                    }
                } else if let Some(error) = bridge::stop_safe() {
                    Response::Error(error)
                } else {
                    RUNNING.store(false, Ordering::Relaxed);
                    Response::Ok
                }
            }
        };
        let payload = serde_json::to_vec(&response)?;
        writer.write_all(&payload).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;

        if quit_flag {
            let _ = shutdown_tx.send(());
            return Ok(());
        }
    }

    Ok(())
}
