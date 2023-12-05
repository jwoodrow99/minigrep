use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = parse_config(&args);

    match fs::read_to_string(config.file_path) {
        Ok(contents) => {
            println!("With text: \n{contents}");
        }
        Err(_) => println!("Cannot read this file."),
    };
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query: String = match &args.get(1) {
        Some(value) => value.to_string(),
        None => panic!("No search query provided."),
    };

    let file_path: String = match &args.get(2) {
        Some(value) => value.to_string(),
        None => panic!("No file path provided."),
    };

    return Config { query, file_path };
}
