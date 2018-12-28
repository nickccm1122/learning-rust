use std::env;
use std::process;

use ch12_minigrep;
use ch12_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = ch12_minigrep::run(config) {
        eprintln!("Application Error: {}", e);

        process::exit(1);
    }
}
