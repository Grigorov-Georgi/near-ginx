mod cli;

fn main() {
    let args = cli::build_cli();

    println!("listen on: {}", args.listen);
    println!("target address: {}", args.target);
    println!("auto-discovery: {}", args.auto_discovery);
    println!("log level: {}", args.log_level);

    if let Some(file) = args.config {
        println!("config file path: {}", file);
    } else {
        println!("config file is empty");
    }
}
