use rmcp::handler::server::tool::ToolRouter;

use crate::preferences::Preferences;

pub mod loc;
pub mod menu_data;
mod router;

pub struct WosGibtsnHeidService {
    preferences: Preferences,
    tool_router: ToolRouter<Self>,
}
