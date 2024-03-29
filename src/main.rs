use mingrep::{config::Config, run};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Couldn't parse arguments: {err}");
        process::exit(1);
    });

    run(config);
}
