#[cfg(not(windows))]
use libc::{uid_t, gid_t};
use log::LogLevelFilter;
use std::path::PathBuf;
use serde::{Deserializer, Deserialize};

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
  pub logging: Logging,
  pub connection: Connection,
  pub certificate: Certificate,
  pub daemon: Daemon
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Logging {
  #[serde(deserialize_with = "de_log_level")]
  #[serde(default = "info")]
  pub level: LogLevelFilter,
  pub file: Option<PathBuf>
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Connection {
  pub hostname: Option<String>,
  pub port: Option<u16>,
  pub name: Option<String>,
  pub reconnect: Option<bool>,
  pub reconnect_period: Option<u64>
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
  pub user: Option<Uid>,
  pub group: Option<Gid>
}

fn de_log_level<'de, D>(d: D) -> Result<LogLevelFilter, D::Error>
  where D: Deserializer<'de>
{
  use serde::de::Error;
  match String::deserialize(d)?.to_lowercase().as_ref() {
    "off" => Ok(LogLevelFilter::Off),
    "trace" => Ok(LogLevelFilter::Trace),
    "debug" => Ok(LogLevelFilter::Debug),
    "info" => Ok(LogLevelFilter::Info),
    "warn" => Ok(LogLevelFilter::Warn),
    "error" => Ok(LogLevelFilter::Error),
    x => Err(D::Error::unknown_variant(x, &["off", "trace", "debug", "info", "warn", "error"]))
  }
}

fn info() -> LogLevelFilter {
  LogLevelFilter::Info
}

impl Default for Logging {
  fn default() -> Logging {
    Logging {
      level: info(),
      file: Default::default()
    }
  }
}
