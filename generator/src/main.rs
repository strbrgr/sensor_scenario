use std::{env, process::exit, str::FromStr, thread::sleep, time::Duration};

use generator::sensor::{SensorType, generate_sensor_reading};

struct Config {
    sensor_type: SensorType,
    frequency: u8,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            3 => {
                let sensor_type = SensorType::from_str(&args[1])?;

                let frequency = args[2]
                    .parse::<u8>()
                    .map_err(|_| "<frequency> needs to be between 0-255.")?;
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

    run(config);
}

fn run(config: Config) {
    loop {
        let reading = generate_sensor_reading(&config.sensor_type);
        println!("{reading}");
        let duration = Duration::new(config.frequency as u64, 0);
        sleep(duration);
    }
}
