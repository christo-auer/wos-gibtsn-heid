use crate::abbreviations::all::*;
use crate::constants::BASE_URL;
use crate::preferences::Preferences;
use crate::tool::menu_data::MenuItem;
use chrono::{Datelike, Local};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    AnnotateAble, CallToolResult, Content, ListResourcesResult, PaginatedRequestParam,
    ProtocolVersion, RawResource, ReadResourceRequestParam, ReadResourceResult, ResourceContents,
    ServerCapabilities, ServerInfo,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData, RoleServer, ServerHandler, tool_handler};
use rmcp::{schemars, tool, tool_router};
use serde_json::json;

use crate::tool::WosGibtsnHeidService;

const RESOURCE_URI_ABBREVIATIONS: &str = "text://wos-gibtsn-heid/abbreviations";

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
struct ParametersFetchMenu {
    #[schemars(description = r#"
         - location for which to fetch the menu, if none is given and a location is stored, it is used.
         - otherwise, you must query the user for a location. 
         - do NOT provide a default location, instead ask for the city AND type (Mensa Cafeteria)
         "#)]
    location: Option<Location>,

    #[schemars(
        description = "GERMAN calendar week, if the user asks for the current week, don't pass this parameter"
    )]
    calendar_week: Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
struct ParametersFetchMenuForWeek {
    #[schemars(
        description = "location for which to fetch the menu, if none is given and a location is stored, it is used. otherwise, you must query the user for a location"
    )]
    location: Option<Location>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
struct ResultFetchMenu {
    #[schemars(description = "menu items as JSON")]
    items: Vec<MenuItem>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
struct ResultGetPreferences {
    #[schemars(description = "preferences of the user")]
    location: Option<Preferences>,
}

#[tool_router]
impl WosGibtsnHeidService {
    pub fn new(preferences: Preferences) -> WosGibtsnHeidService {
        Self {
            preferences,
            tool_router: Self::tool_router(),
        }
    }

    fn get_calendar_week(&self, days: Option<i64>) -> u32 {
        (Local::now() + chrono::Duration::days(days.unwrap_or(0)))
            .iso_week()
            .week()
    }

    fn build_url(&self, location_id: String, week: u32) -> String {
        BASE_URL.to_string() + location_id.as_str() + "/" + week.to_string().as_str() + ".csv"
    }

    async fn fetch_menu_internal(
        &self,
        location: Option<Location>,
        calendar_week: Option<u32>,
    ) -> Result<CallToolResult, ErrorData> {
        let Some(location) = location.or(self.preferences.location.clone()) else {
            return Ok(CallToolResult::error(vec![Content::text(
                "no location is given. ask the user which location should be queried",
            )]));
        };

        if location == Location::ClaudeWorkaroundForEmptyParameter {
            return Ok(CallToolResult::error(vec![Content::text(
                "no location is given. ask the user which location should be queried",
            )]));
        }

        let location_id = location_to_id(&location);

        let week = calendar_week.unwrap_or(self.get_calendar_week(None));

        let url_str = self.build_url(location_id, week);

        let request = reqwest::get(url_str).await;
        let Ok(response) = request else {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "unable to retrieve menu: {}",
                request.unwrap_err()
            ))]));
        };

        if response.status() == 404 {
            return Ok(CallToolResult::error(vec![Content::text(
                "the menu is not available for this week and location",
            )]));
        }

        let text = response.text().await;
        let Ok(csv) = text else {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "cannot retrieve response: {}",
                text.unwrap_err()
            ))]));
        };

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(csv.as_bytes());

        let Ok(records) = reader.deserialize().collect::<Result<Vec<MenuItem>, _>>() else {
            return Ok(CallToolResult::error(vec![
                Content::text("unable to convert data to JSON. You receive the raw CSV instead"),
                Content::text(csv),
            ]));
        };

        Ok(CallToolResult::structured(json!(ResultFetchMenu {
            items: records
        })))
    }

    #[tool(
        description = "CALL THIS BEFORE ANYTHING ELSE. returns the user's preferences. Execute this function BEFORE fetching the menu for the first time and apply these preferences unless told otherwise. IMPORTANT: If values are marked as Unknown give the user a warning and list alle Unknown values and tell the user to check the configuration of this tool."
    )]
    async fn get_preferences(&self) -> Result<CallToolResult, ErrorData> {
        Ok(CallToolResult::success(vec![
            Content::json(&self.preferences).unwrap(),
            Content::text(format!(
                "{}{}{}{}",
                Allergen::describe_all("Allergens:"),
                Ingredient::describe_all("Ingredients:"),
                Indicator::describe_all("Indicators:"),
                Location::describe_all("Locations:"),
            )),
        ]))
    }

    #[tool(
        description = "fetches the menu for the location and calendar week and returns it as a JSON or the raw CSV content if the CSV could not be parsed"
    )]
    async fn fetch_menu(
        &self,
        Parameters(ParametersFetchMenu {
            location,
            calendar_week,
        }): Parameters<ParametersFetchMenu>,
    ) -> Result<CallToolResult, ErrorData> {
        self.fetch_menu_internal(location, calendar_week).await
    }

    #[tool(
        description = "fetches the menu for the location and this week and returns it as a JSON or the raw CSV content if the CSV could not be parsed"
    )]
    async fn fetch_menu_for_this_week(
        &self,
        Parameters(ParametersFetchMenuForWeek { location }): Parameters<ParametersFetchMenuForWeek>,
    ) -> Result<CallToolResult, ErrorData> {
        self.fetch_menu_internal(location, Some(self.get_calendar_week(None)))
            .await
    }

    #[tool(
        description = "fetches the menu for the location and this week and returns it as a JSON or the raw CSV content if the CSV could not be parsed"
    )]
    async fn fetch_menu_for_next_week(
        &self,
        Parameters(ParametersFetchMenuForWeek { location }): Parameters<ParametersFetchMenuForWeek>,
    ) -> Result<CallToolResult, ErrorData> {
        self.fetch_menu_internal(location, Some(self.get_calendar_week(Some(7))))
            .await
    }
}

#[tool_handler]
impl ServerHandler for WosGibtsnHeidService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo { capabilities: ServerCapabilities::builder()
                .enable_tools()
                //.enable_resources()
                .build(),
            instructions: Some("This server provides access to the menus (Speisepläne) of the Studierendenwerk Niederbayern Ostbayern".to_string()),
            protocol_version: ProtocolVersion::V_2024_11_05,
            ..Default::default()
        }
    }

    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, ErrorData> {
        match uri.as_str() {
            RESOURCE_URI_ABBREVIATIONS => Ok(ReadResourceResult {
                contents: vec![ResourceContents::text(
                    format!(
                        "{}{}{}{}",
                        Allergen::describe_all("Allergens:"),
                        Ingredient::describe_all("Ingredients:"),
                        Indicator::describe_all("Indicators:"),
                        Location::describe_all("Locations:"),
                    ),
                    RESOURCE_URI_ABBREVIATIONS.to_string(),
                )],
            }),
            _ => Err(ErrorData::resource_not_found(
                "resource_not_found",
                Some(json!({"uri": uri})),
            )),
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, ErrorData> {
        Ok(ListResourcesResult {
            resources: vec![
                RawResource {
                    description: Some("contains all abbreviations of all indicators/ingredients/allergens in the format `symbol: description`".into()),
                    uri: RESOURCE_URI_ABBREVIATIONS.into(),
                    name: "Abkürzungen".into(),
                    mime_type: Some("text".into()),
                    size: Some(0),
                }.no_annotation(),
            ],
            next_cursor: None,
        })
    }
}
