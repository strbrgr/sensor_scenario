use std::{
    env,
    process::{self, exit},
    str::FromStr,
};

struct Config {
    sensor_type: SensorType,
    frequency: String,
}

enum SensorType {
    Temperature,
    Humidity,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        exit(1);
    });

    run(config);
}

fn run(config: Config) {
    todo!()
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            3 => {
                let sensor_type = SensorType::from_str(&args[1].clone())
                    .map_err(|_| "Passed in <sensor_type> is not an option.")?;

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

#[derive(Debug, PartialEq, Eq)]
struct ParseSensorTypeError;

impl FromStr for SensorType {
    type Err = ParseSensorTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "temperature" => Ok(SensorType::Temperature),
            "humidity" => Ok(SensorType::Humidity),
            _ => Err(ParseSensorTypeError),
        }
    }
}
