use std::{fs, error::Error, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    if results.len() == 0 {
        println!("No results found for '{}'", config.query);
    }

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("CASE_INSENSITIVE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn search_case_sensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_config() {
        let args = vec![
            String::from("minigrep"),
            String::from("query"),
            String::from("file_path"),
        ];

        let config = Config::build(&args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file_path");
    }

    #[test]
    fn test_build_config_not_enough_args() {
        let args = vec![
            String::from("minigrep"),
            String::from("query"),
        ];

        let config = Config::build(&args);

        assert_eq!(config.is_err(), true);
    }

    #[test]
    fn test_search_case_sensitive() {
        let query = "duct";
        let contents = "\n Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "rUsT";
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}