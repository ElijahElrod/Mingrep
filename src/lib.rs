pub mod config;

use config::Config;
use std::fs;

pub fn run(config: Config) {

    let contents = fs::read_to_string(config.file_path).expect("Failed to read file");
    println!("With contents: \n{}", contents);
}