use clap::ValueEnum;
use rmcp::schemars;

#[derive(
    Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema, ValueEnum,
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
}

pub fn location_to_id(location: &Location) -> String {
    serde_json::to_string(&location)
        .unwrap()
        .trim_matches('"')
        .to_string()
}
