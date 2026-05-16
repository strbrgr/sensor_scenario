use rand::random_range;
use std::str::FromStr;

pub enum SensorType {
    Temperature,
    Humidity,
}

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

pub fn generate_sensor_reading(sensor_type: &SensorType, sensor_id: &i32) -> String {
    match sensor_type {
        SensorType::Temperature => {
            let unit = "c";
            let temperature = random_range(-10..=42);
            let sensor_reading = format!(
                r#"{{"id": "temp-{}", "temp":"{}","unit":"{}"}}"#,
                sensor_id, temperature, unit
            );
            sensor_reading
        }
        SensorType::Humidity => {
            let humidity = random_range(0.00..=99.99);
            let sensor_reading = format!(
                r#"{{"id": "humidity-{}", "humidity":{:.1}}}"#,
                sensor_id, humidity
            );
            sensor_reading
        }
    }
}
