use std::{convert::Infallible, str::FromStr};

use rmcp::schemars;
use strum_macros::EnumIter;

use crate::abbreviations::{Abbrevivation, AsUnknown, ParseWithUnknown};

#[derive(Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema, EnumIter)]
pub enum Indicator {
    #[schemars(description = "vegetarisch")]
    V,
    #[schemars(description = "vegan")]
    VG,
    #[schemars(description = "Schwein")]
    S,
    #[schemars(description = "Rind")]
    R,
    #[schemars(description = "Geflügel")]
    G,
    #[schemars(description = "Fisch")]
    F,
    #[schemars(description = "Alkohol")]
    A,
    #[schemars(description = "bio")]
    B,
    #[schemars(description = "Klimateller (planetary health diet)")]
    PHD,

    #[schemars(description = "The user defined an UNKNOWN indicator")]
    #[strum(disabled)]
    Unknown(String),

    #[strum(disabled)]
    ClaudeWorkaroundForEmptyParameter,
}

impl AsUnknown for Indicator {
    fn unknown(s: String) -> Self {
        Self::Unknown(s)
    }
}

impl FromStr for Indicator {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq("${user_config.preferred_indicators}") {
            return Ok(Indicator::ClaudeWorkaroundForEmptyParameter);
        }
        Ok(Self::parse_with_unknown(s))
    }
}

impl Abbrevivation for Indicator {
    fn describe(&self) -> String {
        use Indicator::*;
        match self {
            V => "vegetarisch",
            VG => "vegan",
            S => "Schwein",
            R => "Rind",
            G => "Geflügel",
            F => "Fisch",
            A => "Alkohol",
            B => "bio",
            PHD => "Klimateller (planetary health diet)",
            ClaudeWorkaroundForEmptyParameter => "just IGNORE this value",
            Unknown(_) => "unknown indicator",
        }
        .to_string()
    }
}

// impl FromStr for Indicator {
//     type Err = Infallible;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let Ok(indicator) = serde_json::from_str(&format!("\"{}\"", s.to_uppercase())) else {
//             return Ok(Indicator::Unknown(s.to_string()));
//         };
//         Ok(indicator)
//     }
// }
