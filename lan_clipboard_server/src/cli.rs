use config::Config;
use clap::{App, Arg, ArgMatches};
use toml;
use std::fs::File;
use std::io::Read;

pub fn process_args<'a>() -> ArgMatches<'a> {
  App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .version_message("print version information")
    .help_message("print help information")
    .arg(Arg::with_name("config")
      .short("c")
      .long("config")
      .help("the path of the configuration file to use")
      .value_name("path")
      .takes_value(true))
    .arg(Arg::with_name("hostname")
      .short("H")
      .long("hostname")
      .help("the hostname to bind to")
      .value_name("hostname")
      .takes_value(true))
    .arg(Arg::with_name("port")
      .short("p")
      .long("port")
      .help("the port to bind to")
      .value_name("port")
      .takes_value(true))
    .arg(Arg::with_name("cert_file")
      .short("C")
      .long("certificate")
      .alias("cert")
      .help("the file containing the certificate chain in PEM format")
      .value_name("path")
      .takes_value(true))
    .arg(Arg::with_name("key_file")
      .short("k")
      .long("key")
      .help("the file containing the certificate's key in PEM format")
      .value_name("path")
      .takes_value(true))
    .arg(Arg::with_name("max_size")
      .short("m")
      .long("max-size")
      .help("the maximum size the clipboard may be in bytes")
      .value_name("bytes")
      .takes_value(true))
    .arg(Arg::with_name("daemon")
      .short("d")
      .long("daemon")
      .help("turn on daemon mode")
      .conflicts_with("no_daemon"))
    .arg(Arg::with_name("no_daemon")
      .short("D")
      .long("no-daemon")
      .help("turn off daemon mode (default unless otherwise specified in configuration file)"))
    .get_matches()
}

pub fn load_config<'a>(args: &ArgMatches<'a>) -> Result<Config, String> {
  let mut config: Config = match args.value_of("config") {
    Some(loc) => match File::open(loc) {
      Ok(mut f) => {
        let mut content = String::new();
        f.read_to_string(&mut content)
          .map_err(|e| format!("could not read configuration file at {}: {}", loc, e))?;
        toml::from_str(&content)
          .map_err(|e| format!("could not parse configuration file at {}: {}", loc, e))?
      },
      Err(e) => return Err(format!("could not open configuration file at {}: {}", loc, e))
    },
    None => Default::default()
  };

  config.connection.hostname = args.value_of("hostname").map(Into::into).or(config.connection.hostname);
  if let Some(port) = args.value_of("port") {
    match port.parse() {
      Ok(p) => config.connection.port = Some(p),
      Err(e) => return Err(format!("could not parse port from \"{}\": {}", port, e))
    }
  }

  config.certificate.file = args.value_of("certificate").map(Into::into).or(config.certificate.file);
  config.certificate.key = args.value_of("key_file").map(Into::into).or(config.certificate.key);

  if let Some(max_size) = args.value_of("max_size") {
    match max_size.parse() {
      Ok(m) => config.clipboard.max_size = Some(m),
      Err(e) => return Err(format!("could not parse max size from \"{}\": {}", max_size, e))
    }
  }

  if args.is_present("daemon") {
    config.daemon.enabled = Some(true);
  }
  if args.is_present("no-daemon") {
    config.daemon.enabled = Some(false);
  }

  if config.connection.hostname.is_none() {
    return Err("no hostname was specified".into());
  }
  if config.connection.port.is_none() {
    return Err("no port was specified".into());
  }
  if config.certificate.file.is_none() {
    return Err("no certificate chain file was specified".into());
  }
  if config.certificate.key.is_none() {
    return Err("no certificate key file was specified".into());
  }
  Ok(config)
}
