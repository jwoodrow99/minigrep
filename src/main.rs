use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(2) {
        Some(value) => {
            match args.get(1) {
                Some(value) => println!("Searching for {}", value),
                None => println!("No search string."),
            }

            match fs::read_to_string(&value) {
                Ok(contents) => {
                    println!("With text: \n{contents}");
                }
                Err(_) => println!("Cannot read this file."),
            }
        }
        None => println!("No file given."),
    }
}
