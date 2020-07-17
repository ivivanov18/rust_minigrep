use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap();
    if let Err(err) = minigrep::run(config) {
        println!("Application error {}", err);
        process::exit(1);
    }
}
