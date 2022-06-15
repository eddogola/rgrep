use std::fs;
use std::error::Error;

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("insufficient arguments supplied");
        }
        Ok(Config {
            query: &args[1],
            filename: &args[2],
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    println!("With text:\n {}", content);

    Ok(())
}