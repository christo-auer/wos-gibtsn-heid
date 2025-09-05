use crate::tool::loc::Location;
use clap::Parser;
use rmcp::schemars;

#[derive(Parser, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[command(version, about)]
pub struct Preferences {
    #[arg(
        long,
        value_name = "LOCATION",
        help = "default-location",
        ignore_case = true
    )]
    #[schemars(description = "default location for which for menu should be fetched")]
    pub location: Option<Location>,

    #[arg(
        long,
        value_delimiter = ',',
        value_name = "LIST-OF-ALLERGENES",
        help = r#"comma-separated list of allgenes to avoid, e.g, AA,E,L"#,
        ignore_case = true
    )]
    #[schemars(description = "list of allergenes that MUST BE AVOIDED or WARNED about ")]
    pub avoid_allergens: Option<Vec<String>>,

    #[arg(
        long,
        value_delimiter = ',',
        value_name = "LIST-OF-INGREDIENTS",
        help = r#"comma-separated list of ingredients to avoid, e.g, 1,4,2"#,
        ignore_case = true
    )]
    #[schemars(description = "list of ingredients that MUST BE AVOIDED or WARNED about ")]
    pub avoid_ingredients: Option<Vec<String>>,

    #[arg(
        long,
        value_delimiter = ',',
        value_name = "LIST-OF-INDICATORS",
        help = r#"comma-separated list of preferred indicators, e.g, V,VG"#,
        ignore_case = true
    )]
    #[schemars(
        description = "preferred indicators to list, emphasize selection of dishes according to this list"
    )]
    pub preferred_indicators: Option<Vec<String>>,

    #[serde(skip_serializing)]
    #[arg(long, help = "list available locations")]
    pub list_locations: bool,

    #[serde(skip_serializing)]
    #[arg(long, help = "list available allergens")]
    pub list_allergens: bool,

    #[serde(skip_serializing)]
    #[arg(long, help = "list available ingredients")]
    pub list_ingredients: bool,

    #[serde(skip_serializing)]
    #[arg(long, help = "list available indicators")]
    pub list_indicators: bool,
}
