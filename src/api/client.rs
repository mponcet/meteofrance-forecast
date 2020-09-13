use crate::api::forecast::Forecast;
use crate::api::rainforecast::RainForecast;
use crate::config;
use restson::{Error, RestClient};

pub struct Client {
    rain_rest_client: RestClient,
    forecast_rest_client: RestClient,
}

impl Client {
    pub fn new() -> Self {
        let rain_rc = match RestClient::new("http://webservice.meteofrance.com") {
            Ok(rc) => rc,
            Err(_) => panic!("Coult not create rain rest client"),
        };

        let forecast_rc = match RestClient::new("http://ws.meteofrance.com") {
            Ok(rc) => rc,
            Err(_) => panic!("Coult not create rain rest client"),
        };

        Client {
            rain_rest_client: rain_rc,
            forecast_rest_client: forecast_rc,
        }
    }

    pub fn get_rain_forecast(
        &mut self,
        latitude: f32,
        longitude: f32,
    ) -> Result<RainForecast, Error> {
        let (latitude, longitude) = (latitude.to_string(), longitude.to_string());

        let query = vec![
            ("token", config::METEOFRANCE_WS_TOKEN),
            ("lat", &latitude),
            ("lon", &longitude),
        ];

        self.rain_rest_client.get_with((), &query)
    }

    pub fn get_forecast(&mut self, insee_code: u32) -> Result<Forecast, Error> {
        self.forecast_rest_client.get(insee_code)
    }
}

#[test]
fn test_get_rain_forecast() {
    let mut client = Client::new();
    match client.get_rain_forecast(47.115537, -2.104171) {
        Ok(rs) => assert_eq!(rs.position.lat, 47.115537),
        Err(_) => panic!("Could not get rain forecast"),
    }
}

#[test]
fn test_get_forecast() {
    let mut client = Client::new();
    match client.get_forecast(75056) {
        Ok(rs) => assert_eq!(rs.ville.nom, "Paris"),
        Err(_) => panic!("Could not get forecast"),
    }
}
