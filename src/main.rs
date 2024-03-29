
use std::{env, process};
use mingrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Couldn't parse arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = mingrep::run(config) {
        println!("Error: {e}");
        process::exit(1);
    }
}
