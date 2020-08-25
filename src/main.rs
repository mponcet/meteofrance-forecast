extern crate restson;
#[macro_use]
extern crate serde_derive;

use std::env;
use restson::RestClient;

use crate::api::rainforecast::RainForecast;

mod api;
mod config;


fn help(argv0: &str) {
    println!("{} [lat] [lon]", argv0);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let mut client = RestClient::new("http://webservice.meteofrance.com").unwrap();

            let query = vec![("token", config::METEOFRANCE_WS_TOKEN),
                             ("lat", &args[1]),
                             ("lon", &args[2])];

            let data: RainForecast = client.get_with((), &query).unwrap();
            println!("data: {:?}", data);
        },
        _ => {
            help(&args[0]);
            return
        },
    };

}

