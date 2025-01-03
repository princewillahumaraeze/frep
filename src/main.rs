use std::{env, process};
use frep::Config;

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem Parsing Arguments: {err}");
        process::exit(1)});

    println!("Searching for \"{}\"", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = frep::run(config){
        eprintln!("Application Error: {e}");
        process::exit(1);
    }

}

