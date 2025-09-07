use std::{convert::Infallible, str::FromStr};

use rmcp::schemars;
use strum_macros::EnumIter;

use crate::abbreviations::{Abbrevivation, AsUnknown, ParseWithUnknown};

#[derive(
    Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema, EnumIter,
)]
pub enum Location {
    #[schemars(description = "Hochschule Landshut Mensa")]
    #[serde(rename = "HS-LA")]
    HSLaMensa,

    #[schemars(description = "Hochschule Landshut Cafeteria")]
    #[serde(rename = "HS-LA-Cafeteria")]
    HSLaCafeteria,

    #[schemars(description = "Universität Regensburg Mensa")]
    #[serde(rename = "UNI-R")]
    UniRMensa,

    #[schemars(description = "Universität Regensburg Mensa - Gästesaal")]
    #[serde(rename = "UNI-R-Gs")]
    UniRGaestesaal,

    #[schemars(description = "Universität Regensburg Cafeteria PT")]
    #[serde(rename = "Cafeteria-PT")]
    UniRCafeteriaPT,

    #[schemars(description = "Universität Regensburg Cafeteria Chemie")]
    #[serde(rename = "Cafeteria-Chemie")]
    UniRCafeteriaChemie,

    #[schemars(description = "Universität Regensburg Cafeteria Sammelgebäude")]
    #[serde(rename = "Cafeteria-Sammelgebaeude")]
    UniRCafeteriaSammel,

    #[schemars(description = "Universität Regensburg Cafeteria Sport")]
    #[serde(rename = "Cafeteria-Sport")]
    UniRCafeteriaSport,

    #[schemars(description = "OTH Regensburg Mensa Seybothstraße (Mittags)")]
    #[serde(rename = "HS-R-tag")]
    OthRMensa,

    #[schemars(description = "OTH Regensburg Mensa Seybothstraße (Abends)")]
    #[serde(rename = "HS-R-abend")]
    OthRMensaAbend,

    #[schemars(description = "OTH Regensburg Mensa Prüfeningerstraße (Mittags)")]
    #[serde(rename = "Cafeteria-Pruefening")]
    OthRMensaPruefening,

    #[schemars(description = "Universität Passau Mensa")]
    #[serde(rename = "UNI-P")]
    UniPMensa,

    #[schemars(description = "Universität Passau Cafeteria Nikolakloster")]
    #[serde(rename = "Cafeteria-Nikolakloster")]
    UniPCafeteriaNikolakloster,

    #[schemars(description = "TH Deggendorf Mensa")]
    #[serde(rename = "HS-DEG")]
    ThDegMensa,

    #[schemars(description = "TH Deggendorf-Cham")]
    #[serde(rename = "TH-DEG-Cham")]
    TcCham,

    #[schemars(description = "European Campus Pfarrkirchen")]
    #[serde(rename = "HS-PAN")]
    EcPfarrkirchen,

    #[schemars(description = "TUM Campus Straubing")]
    #[serde(rename = "HS-SR")]
    TumStraubing,

    #[schemars(description = "The user defined an unknown location ")]
    #[strum(disabled)]
    Unknown(String),

    #[strum(disabled)]
    ClaudeWorkaroundForEmptyParameter,
}

impl FromStr for Location {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq("${user_config.location}") {
            return Ok(Location::ClaudeWorkaroundForEmptyParameter);
        }
        Ok(Self::parse_with_unknown(s))
    }
}

pub fn location_to_id(location: &Location) -> String {
    serde_json::to_string(&location)
        .unwrap()
        .trim_matches('"')
        .to_string()
}

impl AsUnknown for Location {
    fn unknown(s: String) -> Self {
        Self::Unknown(s)
    }
}

impl Abbrevivation for Location {
    fn describe(&self) -> String {
        use Location::*;
        match self {
            HSLaMensa => "Hochschule Landshut Mensa".to_string(),
            HSLaCafeteria => "Hochschule Landshut Cafeteria".to_string(),
            UniRMensa => "Universität Regensburg Mensa".to_string(),
            UniRGaestesaal => "Universität Regensburg Mensa - Gästesaal".to_string(),
            UniRCafeteriaPT => "Universität Regensburg Cafeteria PT".to_string(),
            UniRCafeteriaChemie => "Universität Regensburg Cafeteria Chemie".to_string(),
            UniRCafeteriaSammel => "Universität Regensburg Cafeteria Sammelgebäude".to_string(),
            UniRCafeteriaSport => "Universität Regensburg Cafeteria Sport".to_string(),
            OthRMensa => "OTH Regensburg Mensa Seybothstraße (Mittags)".to_string(),
            OthRMensaAbend => "OTH Regensburg Mensa Seybothstraße (Abends)".to_string(),
            OthRMensaPruefening => "OTH Regensburg Mensa Prüfeningerstraße (Mittags)".to_string(),
            UniPMensa => "Universität Passau Mensa".to_string(),
            UniPCafeteriaNikolakloster => "Universität Passau Cafeteria Nikolakloster".to_string(),
            ThDegMensa => "TH Deggendorf Mensa".to_string(),
            TcCham => "TH Deggendorf-Cham".to_string(),
            EcPfarrkirchen => "European Campus Pfarrkirchen".to_string(),
            TumStraubing => "TUM Campus Straubing".to_string(),
            ClaudeWorkaroundForEmptyParameter => "just IGNORE this value".to_string(),
            Unknown(unknown) => format!("unknown location: {}", unknown).to_string(),
        }
    }
}
