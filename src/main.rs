use std::env;
use std::process;

use rgrep::Config;

const CASE_SENSITIVE_VAR: &'static str = "CASE_INSENSITIVE";

fn main() {
    // get config values
    let args: Vec<String> = env::args().collect();
    let case_sensitive = env::var(CASE_SENSITIVE_VAR).is_err();

    let config = Config::new(&args, case_sensitive).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    println!("searching for `{}`", config.query);
    println!("In file: {}", config.filename);

    if let Err(e) = rgrep::run(&config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
