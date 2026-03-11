use std::str::FromStr;

pub enum SensorType {
    Temperature,
    Humidity,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseSensorTypeError;

impl FromStr for SensorType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "temperature" => Ok(SensorType::Temperature),
            "humidity" => Ok(SensorType::Humidity),
            _ => Err("Passed in <sensor_type> is not an option."),
        }
    }
}
