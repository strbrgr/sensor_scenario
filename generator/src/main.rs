use std::{env, error::Error, process::exit, str::FromStr};

use generator::sensor::SensorType;

struct Config {
    sensor_type: SensorType,
    frequency: u8,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            3 => {
                let sensor_type = SensorType::from_str(&args[1])
                    .map_err(|_| "Passed in <sensor_type> is not an option.")?;

                let frequency = args[2]
                    .parse::<u8>()
                    .map_err(|_| "Number needs to be between 0-255.")?;
                Ok(Config {
                    sensor_type,
                    frequency,
                })
            }
            _ => Err("Usage: <sensor type> <frequency>"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        exit(1);
    });

    let _ = run(config);
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}
