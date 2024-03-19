use std::env;
use std::process;

use tp1_grep_rustico::Config;

fn main() {
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problema al parsear argumentos: {err}");
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.file_path);

    if let Err(e) = tp1_grep_rustico::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}