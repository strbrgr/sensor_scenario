use std::{env, process::exit};

struct Config {
    sensor_type: String,
    frequency: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            3 => {
                let sensor_type = args[1].clone();
                let frequency = args[2].clone();
                Ok(Config {
                    sensor_type,
                    frequency,
                })
            }
            _ => Err("Usage: <sensor type> <frequency>"),
        }
    }
}
