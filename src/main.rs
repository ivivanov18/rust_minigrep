use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config: Config = parse_config(&args);

    let contents = fs::read_to_string(config.filename).expect("Error while reading file");
    println!("{:?}", contents);
}

struct Config {
    query: String,
    filename: String
}

fn parse_config(args: &[String]) -> Config{
    let query = args[1].clone();
    let filename = args[2].clone();
    Config {query, filename}
}
