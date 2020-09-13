use chrono::{serde::ts_seconds, DateTime, Utc};
use restson::{Error, RestPath};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum RainIntensity {
    NoRain = 1,
    Light = 2,
    Moderate = 3,
    Heavy = 4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub lat: f32,
    pub lon: f32,
    pub alti: i32,
    pub name: String,
    pub country: String,
    pub dept: String,
    pub timezone: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    #[serde(with = "ts_seconds")]
    pub dt: DateTime<Utc>,
    pub rain: RainIntensity,
    pub desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RainForecast {
    pub position: Position,
    pub updated_on: u32,
    pub quality: i32,
    pub forecast: Vec<Forecast>,
}

impl RestPath<()> for RainForecast {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("rain"))
    }
}
