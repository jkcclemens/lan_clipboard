use config::Config;
#[cfg(not(windows))]
use daemonize::{Daemonize, DaemonizeError};

#[cfg(not(windows))]
pub fn handle_daemon(config: &Config) -> Result<(), DaemonizeError> {
  if config.daemon.enabled != Some(true) {
    return Ok(());
  }
  let mut d = Daemonize::new();
  if let Some(ref pid_file) = config.daemon.pid_file {
    d = d.pid_file(pid_file);
  }
  if let Some(chown_pid_file) = config.daemon.chown_pid_file {
    d = d.chown_pid_file(chown_pid_file);
  }
  if let Some(user) = config.daemon.user {
    d = d.user(user);
  }
  if let Some(group) = config.daemon.group {
    d = d.group(group);
  }
  d
    .working_directory("./")
    .start()
}

#[cfg(windows)]
pub fn handle_daemon(config: &Config) -> Result<(), String> {
  if config.daemon.enabled == Some(true) {
    Err("daemonization is not supported on Windows".into())
  } else {
    Ok(())
  }
}
