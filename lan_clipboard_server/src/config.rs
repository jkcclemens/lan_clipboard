use daemon::{Uid, Gid};
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Config {
  pub connection: Connection,
  pub certificate: Certificate,
  pub clipboard: Clipboard,
  pub daemon: Daemon
}

#[derive(Debug, Default, Deserialize)]
pub struct Connection {
  #[serde(default)]
  pub hostname: Option<String>,
  #[serde(default)]
  pub port: Option<u16>,
  pub max_message_size: u32,
  #[serde(default)]
  pub max_clients: Option<usize>
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
