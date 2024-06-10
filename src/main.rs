extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // イテレータの所有権を直接Config::newに渡す
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // {}を探しています
    println!("Searching for {}", config.query);
    // {}というファイルの中
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
