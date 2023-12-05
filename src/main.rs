use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    match Config::build(&args) {
        Ok(config) => {
            match fs::read_to_string(config.file_path) {
                Ok(contents) => {
                    println!("With text: \n{contents}");
                }
                Err(_) => println!("Cannot read this file."),
            };
        }
        Err(error) => print!("Error: {}", error),
    }
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
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

    fn build(args: &[String]) -> Result<Config, &'static str> {
        let query: String = match &args.get(1) {
            Some(value) => value.to_string(),
            None => return Err("No search query provided."),
        };

        let file_path: String = match &args.get(2) {
            Some(value) => value.to_string(),
            None => return Err("No file path provided."),
        };

        return Ok(Config { query, file_path });
    }
}
