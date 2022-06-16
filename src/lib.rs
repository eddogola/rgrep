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

    for line in search(config.query, &content) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
    let (query_lower, contents_lower) = (query.to_lowercase(), contents.to_lowercase());
    
    // got an error referencing above variables to use in search
    // so I'm going to write a replica of search to get the test to pass
    // then refactor later
    let contents: Vec<&str> = contents.lines().collect();
    let mut results = Vec::new();
    let mut index = 0;
    for line in contents_lower.lines() {
        if line.contains(&query_lower) {
            // results.push(String::from(line));
            println!("{}", index);
            results.push(contents[index]);
        }
        index += 1;
    }

    results
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn case_sensitive_search() {
        let query = "tive";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive_search() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

}