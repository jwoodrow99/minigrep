use std::error::Error;
use std::fs;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    println!("With text: \n{contents}");
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
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

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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
