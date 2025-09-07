use std::{convert::Infallible, str::FromStr};

use rmcp::schemars;
use strum_macros::EnumIter;

use crate::abbreviations::{Abbrevivation, AsUnknown, ParseWithUnknown};

#[derive(Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema, EnumIter)]
pub enum Ingredient {
    #[serde(rename = "1")]
    #[schemars(description = "mit Farbstoff")]
    N1,
    #[serde(rename = "2")]
    #[schemars(description = "mit Konservierungsstoff")]
    N2,
    #[serde(rename = "3")]
    #[schemars(description = "mit Antioxidationsmittel")]
    N3,
    #[serde(rename = "4")]
    #[schemars(description = "mit Geschmacksverstärker")]
    N4,
    #[serde(rename = "5")]
    #[schemars(description = "geschwefelt")]
    N5,
    #[serde(rename = "6")]
    #[schemars(description = "geschwärzt")]
    N6,
    #[serde(rename = "7")]
    #[schemars(description = "gewachst")]
    N7,
    #[serde(rename = "8")]
    #[schemars(description = "mit Phosphat")]
    N8,
    #[serde(rename = "9")]
    #[schemars(description = "mit Süssungsmittel Saccharin")]
    N9,
    #[serde(rename = "10")]
    #[schemars(description = "mit Süssungsmittel Aspartam, enth. Phenylalaninquelle")]
    N10,
    #[serde(rename = "11")]
    #[schemars(description = "mit Süssungsmittel Cyclamat")]
    N11,
    #[serde(rename = "12")]
    #[schemars(description = "mit Süssungsmittel Acesulfam")]
    N12,
    #[serde(rename = "13")]
    #[schemars(description = "chininhaltig")]
    N13,
    #[serde(rename = "14")]
    #[schemars(description = "coffeinhaltig")]
    N14,
    #[serde(rename = "15")]
    #[schemars(description = "enthält Sulfite")]
    N15,
    #[serde(rename = "16")]
    #[schemars(description = "enthält Phenylalanin")]
    N16,

    #[schemars(description = "The user defined an UNKNOWN ingredient")]
    #[strum(disabled)]
    Unknown(String),

    #[strum(disabled)]
    ClaudeWorkaroundForEmptyParameter,
}

impl AsUnknown for Ingredient {
    fn unknown(s: String) -> Self {
        Self::Unknown(s)
    }
}

impl FromStr for Ingredient {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq("${user_config.avoid_ingredients}") {
            return Ok(Ingredient::ClaudeWorkaroundForEmptyParameter);
        }
        Ok(Self::parse_with_unknown(s))
    }
}

impl Abbrevivation for Ingredient {
    fn describe(&self) -> String {
        use Ingredient::*;
        match self {
            N1 => "mit Farbstoff",
            N2 => "mit Konservierungsstoff",
            N3 => "mit Antioxidationsmittel",
            N4 => "mit Geschmacksverstärker",
            N5 => "geschwefelt",
            N6 => "geschwärzt",
            N7 => "gewachst",
            N8 => "mit Phosphat",
            N9 => "mit Süssungsmittel Saccharin",
            N10 => "mit Süssungsmittel Aspartam, enth. Phenylalaninquelle",
            N11 => "mit Süssungsmittel Cyclamat",
            N12 => "mit Süssungsmittel Acesulfam",
            N13 => "chininhaltig",
            N14 => "coffeinhaltig",
            N15 => "enthält Sulfite",
            N16 => "enthält Phenylalanin",
            ClaudeWorkaroundForEmptyParameter => "just IGNORE this value",
            Unknown(_) => "unknown ingredient",
        }
        .to_string()
    }
}
