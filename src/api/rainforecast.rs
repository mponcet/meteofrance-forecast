use chrono::{DateTime, Utc, serde::ts_seconds};
use restson::{Error, RestPath};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[cfg(test)]
use restson::RestClient;
#[cfg(test)]
use crate::config;

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum RainIntensity {
    NoRain = 1,
    Light = 2,
    Moderate = 3,
    Heavy = 4,
}

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    lat: f32,
    lon: f32,
    alti: i32,
    name: String,
    country: String,
    dept: String,
    timezone: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    #[serde(with = "ts_seconds")]
    dt: DateTime<Utc>,
    rain: RainIntensity,
    desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RainForecast {
    position: Position,
    updated_on: u32,
    quality: i32,
    forecast: Vec<Forecast>,
}

impl RestPath<()> for RainForecast {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("rain"))
    }
}

#[test]
fn test_get_rainforecast() {
    let mut client = RestClient::new("http://webservice.meteofrance.com").unwrap();

    let query = vec![("token", config::METEOFRANCE_WS_TOKEN),
                     ("lat", "47.115537"),
                     ("lon", "-2.104171")];

    let result: RainForecast = client.get_with((), &query).unwrap();
    assert_eq!(result.position.lat, 47.115537);
}
