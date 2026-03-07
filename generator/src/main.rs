use std::{
    env,
    process::{self, exit},
};

struct Config {
    sensor_type: String,
    frequency: String,
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
