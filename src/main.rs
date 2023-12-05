use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            println!("With text: \n{contents}");
        }
        Err(_) => println!("Cannot read this file."),
    };
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let file_path = &args[2];

    let query = match &args.get(1) {
        Some(value) => *value,
        None => panic!("No search query provided."),
    };

    let file_path = match &args.get(2) {
        Some(value) => *value,
        None => panic!("No file path provided."),
    };

    return (query, file_path);
}
