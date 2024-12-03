use std::{env, process::exit};

use minigrep::Config;

fn main() {
    // To get the arguments pased in the function

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        exit(1);
    });

    // To Read the file's content:
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: \n {e}");
        exit(1)
    }
}
