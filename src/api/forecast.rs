use chrono::{serde::ts_milliseconds, DateTime, Utc};
use restson::{Error, RestPath};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ville {
    pub nom: String,
    pub latitude: String,
    pub longitude: String,
    #[serde(rename = "couvertPluie")]
    pub couvert_pluie: bool,
    #[serde(rename = "bulletinMontagne")]
    pub bulletin_montagne: bool,
    #[serde(rename = "bulletinCote")]
    pub bulletin_cote: bool,
    pub plage: bool,
    pub montagne: bool,
    pub vigilance: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resume {
    #[serde(with = "ts_milliseconds")]
    pub date: DateTime<Utc>,
    pub jour: u8,
    pub description: Option<String>,
    #[serde(rename = "directionVent")]
    pub direction_vent: Option<String>,
    #[serde(rename = "forceRafales")]
    pub force_rafales: Option<String>,
    #[serde(rename = "temperatureMin")]
    pub temperature_min: Option<String>,
    #[serde(rename = "temperatureMax")]
    pub temperature_max: Option<String>,
    #[serde(rename = "indiceUV")]
    pub indice_uv: Option<String>,
    #[serde(rename = "probaPluie")]
    pub proba_pluie: Option<String>,
    #[serde(rename = "probaNeige")]
    pub proba_neige: Option<String>,
    #[serde(rename = "probaGel")]
    pub proba_gel: Option<String>,
    #[serde(rename = "indiceConfiance")]
    pub indice_confiance: Option<String>,
    #[serde(rename = "temperatureMer")]
    pub temperature_mer: Option<String>,
    #[serde(rename = "etatMer")]
    pub etat_mer: Option<String>,
    pub iso0: Option<String>,
    #[serde(rename = "limitePluieNeige")]
    pub limite_pluie_neige: Option<String>,
}

#[derive(Debug)]
pub struct Forecast {
    pub ville: Ville,
    pub resumes: HashMap<String, Resume>,
    pub previsions: HashMap<String, Resume>,
    pub previsions48h: HashMap<String, Resume>,
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
    where
        D: Deserializer<'de>,
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
