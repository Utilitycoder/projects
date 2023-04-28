use minigrep::{Config, run};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|e| {
        println!("Problem parsing arguments: {}", e);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}