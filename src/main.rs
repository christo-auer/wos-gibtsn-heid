use clap::Parser;
use rmcp::ServiceExt;
use tokio::io::{stdin, stdout};

use crate::tool::{
    WosGibtsnHeidService,
    loc::LOCATIONS_STRING,
    menu_data::{ALLERGENS_STRING, INDICATORS_STRING, INGREDIENTS_STRING},
};
use preferences::Preferences;

mod preferences;
mod tool;

pub mod constants {
    pub const BASE_URL: &str = "https://www.stwno.de/infomax/daten-extern/csv/";
}

fn process_preferences_args(preferences: &Preferences) -> bool {
    if preferences.list_allergens {
        println!("{}", ALLERGENS_STRING.as_str());
        return true;
    }

    if preferences.list_ingredients {
        println!("{}", INGREDIENTS_STRING.as_str());
        return true;
    }

    if preferences.list_indicators {
        println!("{}", INDICATORS_STRING.as_str());
        return true;
    }

    if preferences.list_locations {
        println!("{}", LOCATIONS_STRING.as_str());
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
