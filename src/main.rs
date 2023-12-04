use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(value) => println!("Searching for {}", value),
        None => println!("No search string."),
    }

    match args.get(2) {
        Some(value) => println!("In file {}", value),
        None => println!("No file given."),
    }
}
