#[cfg(not(windows))]
use libc::{uid_t, gid_t};
use std::path::PathBuf;

#[cfg(not(windows))]
type Uid = uid_t;
#[cfg(windows)]
type Uid = u32;

#[cfg(not(windows))]
type Gid = gid_t;
#[cfg(windows)]
type Gid = u32;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Config {
  pub connection: Connection,
  pub certificate: Certificate,
  pub clipboard: Clipboard,
  pub daemon: Daemon
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Connection {
  pub hostname: Option<String>,
  pub port: Option<u16>
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Certificate {
  pub file: Option<PathBuf>,
  pub key: Option<PathBuf>
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Clipboard {
  pub max_size: Option<usize>
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Daemon {
  pub enabled: Option<bool>,
  pub pid_file: Option<PathBuf>,
  pub chown_pid_file: Option<bool>,
  pub user: Option<Uid>,
  pub group: Option<Gid>
}
