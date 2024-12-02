use std::{env, process::exit};

use minigrep::Config;

fn main() {
    // To get the arguments pased in the function
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        exit(1);
    });

    println!("Querying: \"{}\"", config.query);
    println!("Results: \n============");

    // To Read the file's content:
    if let Err(e) = minigrep::run(config) {
        println!("Application error: \n {e}");
        exit(1)
    }
}
