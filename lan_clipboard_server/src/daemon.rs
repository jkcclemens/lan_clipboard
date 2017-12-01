use config::Config;
#[cfg(not(windows))]
use daemonize::{Daemonize, DaemonizeError};
#[cfg(not(windows))]
use libc::{uid_t, gid_t};

#[cfg(not(windows))]
pub type Uid = uid_t;
#[cfg(windows)]
pub type Uid = u32;

#[cfg(not(windows))]
pub type Gid = gid_t;
#[cfg(windows)]
pub type Gid = u32;

#[cfg(not(windows))]
pub struct Daemon(Daemonize<()>);
#[cfg(windows)]
pub struct Daemon;

#[cfg(not(windows))]
impl Daemon {
  pub fn new(config: &Config) -> Result<Option<Daemon>, String> {
    if config.daemon.enabled != Some(true) {
      return Ok(None);
    }

    let mut s = Daemon(Daemonize::new());

    if let Some(ref pid_file) = config.daemon.pid_file {
      s.0 = s.0.pid_file(pid_file);
    }
    if let Some(chown_pid_file) = config.daemon.chown_pid_file {
      s.0 = s.0.chown_pid_file(chown_pid_file);
    }
    if let Some(user) = config.daemon.user {
      s.0 = s.0.user(user);
    }
    if let Some(group) = config.daemon.group {
      s.0 = s.0.group(group);
    }

    Ok(Some(s))
  }

  pub fn start(self) -> Result<(), DaemonizeError> {
    self.0.start()
  }
}

#[cfg(windows)]
impl Daemon {
  #[inline]
  fn err<T>() -> Result<T, String> {
    Err("daemonization is not supported on Windows".into())
  }

  pub fn new(_: &Config) -> Result<Daemon, String> {
    Daemon::err()
  }

  pub fn start(self) -> Result<(), DaemonizeError> {
    Daemon::err()
  }
}
