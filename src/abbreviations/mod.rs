pub mod allergens;
pub mod indicators;
pub mod ingredients;
pub mod locations;

use serde::Serialize;
use strum::IntoEnumIterator;

pub mod all {
    pub use super::allergens::Allergen;
    pub use super::indicators::Indicator;
    pub use super::ingredients::Ingredient;
    pub use super::locations::{Location, location_to_id};
    pub use super::*;
}

pub trait AsUnknown: Sized + for<'de> serde::Deserialize<'de> {
    fn unknown(s: String) -> Self;
}

pub trait ParseWithUnknown: AsUnknown {
    fn parse_with_unknown(s: &str) -> Self {
        serde_json::from_str(&format!("\"{}\"", s.to_uppercase()))
            .unwrap_or(Self::unknown(s.to_string()))
    }
}

pub trait Abbrevivation: IntoEnumIterator + Serialize {
    fn describe(&self) -> String;

    fn abbreviate(&self) -> String {
        serde_json::to_string(&self)
            .unwrap()
            .trim_matches('"')
            .to_string()
    }

    fn describe_all(header: &str) -> String {
        let mut all: String = String::from(header) + "\n";

        for item in Self::iter() {
            all += format!("  {}: {}\n", item.abbreviate(), item.describe()).as_str();
        }

        all
    }
}

impl<T: AsUnknown> ParseWithUnknown for T {}
