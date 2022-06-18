use std::env;
use std::process;

use rgrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    println!("searching for `{}`", config.query);
    println!("In file: {}", config.filename);

    if let Err(e) = rgrep::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
