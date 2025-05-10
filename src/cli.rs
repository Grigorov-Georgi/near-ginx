use clap::{Arg, ArgAction, Command};

pub struct CliArgs {
    pub listen: String,
    pub target: String,
    pub auto_discovery: bool,
    pub config: Option<String>,
    pub log_level: String,
}

pub fn build_cli() -> CliArgs {
    let matches = Command::new("Near-GINX")
        .version("0.0.1")
        .author("Georgi Grigorov <georgi.grigorov@limechain.tech>")
        .about("A simple reverse proxy in Rust")
        .arg(
            Arg::new("listen")
                .short('l')
                .long("listen")
                .default_value("127.0.0.1:8080")
                .help("Socket address to listen to"),
        )
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .required(true)
                .help("Target upstream server (e.g. http://localhost:1000"),
        )
        .arg(
            Arg::new("auto-discovery")
                .short('a')
                .long("auto-discovery")
                .action(ArgAction::SetTrue)
                .help("Enables auto-discovey if set"),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Path to optional configuration file"),
        )
        .arg(
            Arg::new("log-level")
                .long("log")
                .default_value("info")
                .help("Logging level [info, debug, trace]"),
        )
        .get_matches();

    CliArgs {
        listen: matches.get_one::<String>("listen").unwrap().clone(),
        target: matches.get_one::<String>("target").unwrap().clone(),
        auto_discovery: matches.get_flag("auto-discovery"),
        config: matches.get_one::<String>("config").map(String::from),
        log_level: matches.get_one::<String>("log-level").unwrap().clone(),
    }
}
