use rmcp::schemars;

#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct MenuItem {
    #[schemars(description = "date in the format DD.MM.YYYY")]
    #[serde(rename = "datum")]
    date: String,
    #[schemars(description = "day of week as german abbreviation (Mo, Di, Mi, Do, Fr, Sa, So)")]
    #[serde(rename = "tag")]
    day_of_week: String,
    #[schemars(description = r#"group with number (format <Group><Number>)
        Groups are
          - HG Hauptgericht (main dish)
          - B Beilage (side)
          - N Nachspeise (dessert)"#)]
    #[serde(rename = "warengruppe")]
    group: String,
    #[schemars(
        description = r#"name of dish in German with types of ingredients and allergens in parenthesis."#
    )]
    #[serde(rename = "name")]
    name: String,
    #[schemars(description = r#"indicators"#)]
    #[serde(rename = "kennz")]
    indicators: String,
    #[schemars(description = "price for students in EUR")]
    #[serde(rename = "stud")]
    price_students: String,
    #[schemars(description = "price for employees in EUR")]
    #[serde(rename = "bed")]
    price_employees: String,
    #[schemars(description = "price for guests in EUR")]
    #[serde(rename = "gast")]
    price_guests: String,
}
