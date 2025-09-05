use indexmap::IndexMap;
use once_cell::sync::Lazy;
use rmcp::schemars;

fn abbreviation_table_to_string(abbreviations: &IndexMap<&str, &str>) -> String {
    abbreviations
        .iter()
        .map(|(abbr, desc)| format!("{abbr}: {desc}"))
        .reduce(|acc, item| format!("{acc}\n{item}"))
        .unwrap()
}

pub static INGREDIENTS: Lazy<IndexMap<&'static str, &'static str>> = Lazy::new(|| {
    IndexMap::from([
        ("1", "mit Farbstoff"),
        ("2", "mit Konservierungsstoff"),
        ("3", "mit Antioxidationsmittel"),
        ("4", "mit Geschmacksverstärker"),
        ("5", "geschwefelt"),
        ("6", "geschwärzt"),
        ("7", "gewachst"),
        ("8", "mit Phosphat"),
        ("9", "mit Süssungsmittel Saccharin"),
        (
            "10",
            "mit Süssungsmittel Aspartam, enth. Phenylalaninquelle",
        ),
        ("11", "mit Süssungsmittel Cyclamat"),
        ("12", "mit Süssungsmittel Acesulfam"),
        ("13", "chininhaltig"),
        ("14", "coffeinhaltig"),
        ("16", "enthält Sulfite"),
        ("17", "enthält Phenylalanin"),
    ])
});

pub static INGREDIENTS_STRING: Lazy<String> =
    Lazy::new(|| abbreviation_table_to_string(&INGREDIENTS));

pub static ALLERGENS: Lazy<IndexMap<&'static str, &'static str>> = Lazy::new(|| {
    IndexMap::from([
        ("AA", "Weizengluten"),
        ("AB", "Roggengluten"),
        ("AC", "Gerstengluten"),
        ("AD", "Hafergluten"),
        ("AE", "Dinkelgluten"),
        ("AF", "Kamutgluten"),
        ("B", "Krebstiere"),
        ("C", "Eier"),
        ("D", "Fisch"),
        ("E", "Erdnüsse"),
        ("F", "Soja"),
        ("G", "Milch und Milchprodukte"),
        ("HA", "Mandel"),
        ("HB", "Haselnuss"),
        ("HC", "Walnuss"),
        ("HD", "Cashew"),
        ("HE", "Pecannuss"),
        ("HF", "Paranuss"),
        ("HG", "Pistazie"),
        ("HH", "Macadamianuss"),
        ("HI", "Queenslandnuss"),
        ("I", "Sellerie"),
        ("J", "Senf"),
        ("K", "Sesamsamen"),
        ("L", "Schwefeldioxid und Sulfite"),
        ("M", "Lupinen"),
        ("N", "Weichtiere"),
        ("O", "Nitrat"),
        ("P", "Nitritpökelsalz"), 
    ])
});

pub static ALLERGENS_STRING: Lazy<String> = Lazy::new(|| abbreviation_table_to_string(&ALLERGENS));

pub static INDICATORS: Lazy<IndexMap<&'static str, &'static str>> = Lazy::new(|| {
    IndexMap::from([
        ("V", "vegetarisch"),
        ("VG", "vegan"),
        ("S", "Schwein"),
        ("R", "Rind"),
        ("G", "Geflügel"),
        ("F", "Fisch"),
        ("A", "Alkohol"),
        ("B", "bio"),
        ("PHD", "Klimateller (planetary health diet)"),
    ])
});

pub static INDICATORS_STRING: Lazy<String> =
    Lazy::new(|| abbreviation_table_to_string(&INDICATORS));

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
