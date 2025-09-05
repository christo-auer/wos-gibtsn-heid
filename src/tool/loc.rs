use clap::ValueEnum;
use once_cell::sync::Lazy;
use rmcp::schemars;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(
    Copy,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    ValueEnum,
    EnumIter,
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

fn location_to_description(location: &Location) -> String {
    use Location::*;
    match location {
        HSLaMensa => "Hochschule Landshut Mensa",
        HSLaCafeteria => "Hochschule Landshut Cafeteria",
        UniRMensa => "Universität Regensburg Mensa",
        UniRGaestesaal => "Universität Regensburg Mensa - Gästesaal",
        UniRCafeteriaPT => "Universität Regensburg Cafeteria PT",
        UniRCafeteriaChemie => "Universität Regensburg Cafeteria Chemie",
        UniRCafeteriaSammel => "Universität Regensburg Cafeteria Sammelgebäude",
        UniRCafeteriaSport => "Universität Regensburg Cafeteria Sport",
        OthRMensa => "OTH Regensburg Mensa Seybothstraße (Mittags)",
        OthRMensaAbend => "OTH Regensburg Mensa Seybothstraße (Abends)",
        OthRMensaPruefening => "OTH Regensburg Mensa Prüfeningerstraße (Mittags)",
        UniPMensa => "Universität Passau Mensa",
        UniPCafeteriaNikolakloster => "Universität Passau Cafeteria Nikolakloster",
        ThDegMensa => "TH Deggendorf Mensa",
        TcCham => "TH Deggendorf-Cham",
        EcPfarrkirchen => "European Campus Pfarrkirchen",
        TumStraubing => "TUM Campus Straubing",
    }
    .to_string()
}

pub static LOCATIONS_STRING: Lazy<String> = Lazy::new(|| {
    Location::iter()
        .map(|loc| {
            format!(
                "{}: {}",
                location_to_id(&loc),
                location_to_description(&loc)
            )
        })
        .reduce(|accum, next| format!("{accum}\n{next}"))
        .unwrap()
});
