use chrono::{DateTime, Utc, serde::ts_milliseconds};
use restson::{Error, RestPath};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[cfg(test)]
use restson::RestClient;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ville {
    nom: String,
    latitude: String,
    longitude: String,
    #[serde(rename = "couvertPluie")]
    couvert_pluie: bool,
    #[serde(rename = "bulletinMontagne")]
    bulletin_montagne: bool,
    #[serde(rename = "bulletinCote")]
    bulletin_cote: bool,
    plage: bool,
    montagne: bool,
    vigilance: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resume {
    #[serde(with = "ts_milliseconds")]
    date: DateTime<Utc>,
    jour: u8,
    description: Option<String>,
    #[serde(rename = "directionVent")]
    direction_vent: Option<String>,
    #[serde(rename = "forceRafales")]
    force_rafales: Option<String>,
    #[serde(rename = "temperatureMin")]
    temperature_min: Option<String>,
    #[serde(rename = "temperatureMax")]
    temperature_max: Option<String>,
    #[serde(rename = "indiceUV")]
    indice_uv: Option<String>,
    #[serde(rename = "probaPluie")]
    proba_pluie: Option<String>,
    #[serde(rename = "probaNeige")]
    proba_neige: Option<String>,
    #[serde(rename = "probaGel")]
    proba_gel: Option<String>,
    #[serde(rename = "indiceConfiance")]
    indice_confiance: Option<String>,
    #[serde(rename = "temperatureMer")]
    temperature_mer: Option<String>,
    #[serde(rename = "etatMer")]
    etat_mer: Option<String>,
    iso0: Option<String>,
    #[serde(rename = "limitePluieNeige")]
    limite_pluie_neige: Option<String>,
}

#[derive(Debug)]
pub struct Forecast {
    ville: Ville,
    resumes: HashMap<String, Resume>,
    previsions: HashMap<String, Resume>,
    previsions48h: HashMap<String, Resume>,
}

#[derive(Deserialize, Debug)]
struct ForecastDeserialize {
    ville: Ville,
    resumes: HashMap<String, Resume>,
    previsions: HashMap<String, Resume>,
    previsions48h: HashMap<String, Resume>,
}

// Get rid of top level field 'result'
impl<'de> Deserialize<'de> for Forecast {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize, Debug)]
        struct Outer {
            result: ForecastDeserialize,
        }

        let helper = Outer::deserialize(deserializer)?;
        Ok(Forecast {
            ville: helper.result.ville,
            resumes: helper.result.resumes,
            previsions: helper.result.previsions,
            previsions48h: helper.result.previsions48h,
        })
    }
}

impl RestPath<u32> for Forecast {
    fn get_path(insee_code: u32) -> Result<String, Error> {
        Ok(format!("ws/getDetail/france/{}0.json", insee_code))
    }
}

#[test]
fn test_get_forecast() {
    let mut client = RestClient::new("http://ws.meteofrance.com").unwrap();

    let _result: Forecast = client.get(75056).unwrap();
    println!("{:#?}", _result);
}
