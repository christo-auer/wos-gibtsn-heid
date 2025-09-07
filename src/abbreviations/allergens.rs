use std::{convert::Infallible, str::FromStr};

use rmcp::schemars;
use strum_macros::EnumIter;

use crate::abbreviations::{Abbrevivation, AsUnknown, ParseWithUnknown};

#[derive(Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema, EnumIter)]
pub enum Allergen {
    #[schemars(description = "Weizengluten")]
    AA,
    #[schemars(description = "Roggengluten")]
    AB,
    #[schemars(description = "Gerstengluten")]
    AC,
    #[schemars(description = "Hafergluten")]
    AD,
    #[schemars(description = "Dinkelgluten")]
    AE,
    #[schemars(description = "Kamutgluten")]
    AF,
    #[schemars(description = "Krebstiere")]
    B,
    #[schemars(description = "Eier")]
    C,
    #[schemars(description = "Fisch")]
    D,
    #[schemars(description = "Erdnüsse")]
    E,
    #[schemars(description = "Soja")]
    F,
    #[schemars(description = "Milch und Milchprodukte")]
    G,
    #[schemars(description = "Mandel")]
    HA,
    #[schemars(description = "Haselnuss")]
    HB,
    #[schemars(description = "Walnuss")]
    HC,
    #[schemars(description = "Cashew")]
    HD,
    #[schemars(description = "Pecannuss")]
    HE,
    #[schemars(description = "Paranuss")]
    HF,
    #[schemars(description = "Pistazie")]
    HG,
    #[schemars(description = "Macadamianuss")]
    HH,
    #[schemars(description = "Queenslandnuss")]
    HI,
    #[schemars(description = "Sellerie")]
    I,
    #[schemars(description = "Senf")]
    J,
    #[schemars(description = "Sesamsamen")]
    K,
    #[schemars(description = "Schwefeldioxid und Sulfite")]
    L,
    #[schemars(description = "Lupinen")]
    M,
    #[schemars(description = "Weichtiere")]
    N,
    #[schemars(description = "Nitrat")]
    O,
    #[schemars(description = "Nitritpökelsalz")]
    P,
    #[schemars(description = "The user defined an UNKNOWN allergen")]
    #[strum(disabled)]
    Unknown(String),
}

impl AsUnknown for Allergen {
    fn unknown(s: String) -> Self {
        Self::Unknown(s)
    }
}

impl FromStr for Allergen {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse_with_unknown(s))
    }
}

impl Abbrevivation for Allergen {
    fn describe(&self) -> String {
        use Allergen::*;
        match self {
            AA => "Weizengluten",
            AB => "Roggengluten",
            AC => "Gerstengluten",
            AD => "Hafergluten",
            AE => "Dinkelgluten",
            AF => "Kamutgluten",
            B => "Krebstiere",
            C => "Eier",
            D => "Fisch",
            E => "Erdnüsse",
            F => "Soja und Milchprodukte",
            G => "Milch ",
            HA => "Mandel",
            HB => "Haselnuss",
            HC => "Walnuss",
            HD => "Cashew",
            HE => "Pecannuss",
            HF => "Paranuss",
            HG => "Pistazie",
            HH => "Macadamianuss",
            HI => "Queenslandnuss",
            I => "Sellerie",
            J => "Senf",
            K => "Sesamsamen und Sulfite",
            L => "Schwefeldioxid ",
            M => "Lupinen",
            N => "Weichtiere",
            O => "Nitrat",
            P => "Nitritpökelsalz",
            Unknown(_) => "unknown allergen",
        }
        .to_string()
    }
}
