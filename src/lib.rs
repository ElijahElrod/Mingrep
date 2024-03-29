mod config;

pub use config::Config;
use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With contents: \n{}", contents);

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    let mut matched = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            matched.push(line);
        }
    }
    matched
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
        ";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
   }
}