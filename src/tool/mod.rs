use loc::Location;
use rmcp::handler::server::tool::ToolRouter;
use tokio::sync::Mutex;

pub mod loc;
pub mod menu_data;
mod router;

pub struct WosGibtsnHeidService {
    loc: Mutex<Option<Location>>,
    tool_router: ToolRouter<Self>,
}
