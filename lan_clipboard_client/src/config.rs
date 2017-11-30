use libc::{uid_t, gid_t};
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Config {
  pub connection: Connection,
  pub certificate: Certificate,
  pub daemon: Daemon
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Connection {
  pub hostname: Option<String>,
  pub port: Option<u16>,
  pub name: Option<String>
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Certificate {
  pub file: Option<PathBuf>
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Daemon {
  pub enabled: Option<bool>,
  pub pid_file: Option<PathBuf>,
  pub chown_pid_file: Option<bool>,
  pub user: Option<uid_t>,
  pub group: Option<gid_t>
}
