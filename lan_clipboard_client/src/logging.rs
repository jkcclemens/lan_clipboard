use config::Config;
use std::io;
use std::fs::OpenOptions;
use std::error::Error;
use log::SetLoggerError;
use fern::Dispatch;
use chrono::Local;

pub enum LoggerError {
  Logger(SetLoggerError),
  Io(io::Error)
}

impl LoggerError {
  pub fn into_inner(self) -> Box<Error> {
    match self {
      LoggerError::Logger(e) => Box::new(e),
      LoggerError::Io(e) => Box::new(e)
    }
  }
}

pub fn set_up_logger(config: &Config) -> Result<(), LoggerError> {
  let mut d = Dispatch::new()
    .filter(|f| {
      let first = f.target().split("::").next();
      first == Some("lan_clipboard_client") || first == Some("lan_clipboard")
    })
    .format(|out, message, record| {
      out.finish(format_args!(
        "[{}] [{}] {}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        record.level(),
        message
      ))
    })
    .level(config.logging.level)
    .chain(io::stdout());
  if let Some(ref path) = config.logging.file {
    let f = OpenOptions::new()
      .write(true)
      .create(true)
      .append(true)
      .open(path)
      .map_err(LoggerError::Io)?;
    d = d.chain(f);
  }
  d.apply().map_err(LoggerError::Logger)
}
