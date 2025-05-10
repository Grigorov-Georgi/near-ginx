use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("RustReverseProxy")
        .version("0.0.1")
        .author("Georgi Grigorov <georgi.grigorov@limechain.com>")
        .about("A simple reverse proxy in RUST")
        .arg(
            Arg::new("listen")
                .short('l')
                .long("listen")
                .default_value("127.0.0.1:8080")
                .help("Socket address to listen on"),
        )
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .required(true)
                .help("Target upstream server (e.g. http://localhost:3000"),
        )
        .arg(
            Arg::new("auto-discovery")
                .short('a')
                .long("auto-discovery")
                .action(ArgAction::SetTrue)
                .help("Enables auto-discovery if enabled")
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

    let listen = matches.get_one::<String>("listen").unwrap();
    let target = matches.get_one::<String>("target").unwrap();
    let auto_discovery = matches.get_flag("auto-discovery");
    let log_level = matches.get_one::<String>("log-level").unwrap();
    let config = matches.get_one::<String>("config");

    println!("listen on: {}", listen);
    println!("target address: {}", target);
    println!("auto-discovery: {}", auto_discovery);
    println!("log level: {}", log_level);

    if let Some(file) = config {
        println!("config file path: {}", file);
    } else {
        println!("config file is empty");
    }
}
