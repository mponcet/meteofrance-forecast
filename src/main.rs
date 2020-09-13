extern crate restson;
#[macro_use]
extern crate serde_derive;

use std::{env, process};

use crate::api::client::Client;

mod api;
mod config;

fn help() {
    let program = &env::args().collect::<Vec<String>>()[0];
    println!("{} rain [lat] [lon]", program);
    println!("{} forecase [insee_code]", program);
    process::exit(0);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
    }

    let mut client = Client::new();

    match &args[1][..] {
        "rain" => {
            if args.len() != 4 {
                help();
            }

            let latitude: f32 = match args[2].parse() {
                Ok(lat) => lat,
                Err(_) => panic!("Could not parse latitude"),
            };
            let longitude: f32 = match args[3].parse() {
                Ok(lon) => lon,
                Err(_) => panic!("Could not parse longitude"),
            };
            let data = client.get_rain_forecast(latitude, longitude);
            println!("data: {:?}", data);
        }
        "forecast" => {
            if args.len() != 3 {
                help();
            }

            let insee_code: u32 = match args[2].parse() {
                Ok(ic) => ic,
                Err(_) => panic!("Could not parse insee code"),
            };
            let data = client.get_forecast(insee_code);
            println!("data: {:?}", data);
        }
        _ => {
            help();
            return;
        }
    };
}
