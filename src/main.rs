use clap::Parser;
use rmcp::ServiceExt;
use tokio::io::{stdin, stdout};

use crate::{abbreviations::all::*, tool::WosGibtsnHeidService};
use preferences::Preferences;

mod abbreviations;
mod preferences;
mod tool;

pub mod constants {
    pub const BASE_URL: &str = "https://www.stwno.de/infomax/daten-extern/csv/";
}

fn process_preferences_args(preferences: &Preferences) -> bool {
    if let Some(list) = preferences.list.clone() {
        match list.to_lowercase().as_str() {
            "allergens" => println!("{}", Allergen::describe_all("Allergens:")),
            "ingredients" => println!("{}", Ingredient::describe_all("Ingredients:")),
            "indicators" => println!("{}", Indicator::describe_all("Indicators:")),
            "locations" => println!("{}", Location::describe_all("Locations:")),
            _ => {}
        }
        return true;
    }

    false
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let preferences = Preferences::parse();

    if process_preferences_args(&preferences) {
        return Ok(());
    }

    let server: WosGibtsnHeidService = WosGibtsnHeidService::new(preferences);
    let io = (stdin(), stdout());

    let service = server.serve(io).await?;
    service.waiting().await?;
    Ok(())
}
