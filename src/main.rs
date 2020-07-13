use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let (query, filename) = parse_config(&args);

    let contents = fs::read_to_string(filename).expect("Error while reading file");
    println!("{:?}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let filename = &args[2];
    let query = &args[1];
    
    (query, filename)
}
