use clap::Parser;
use rmcp::ServiceExt;
use tokio::io::{stdin, stdout};

use crate::tool::WosGibtsnHeidService;
use cli::Cli;

mod cli;
mod tool;

pub mod constants {
    pub const BASE_URL: &str = "https://www.stwno.de/infomax/daten-extern/csv/";
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let server: WosGibtsnHeidService = WosGibtsnHeidService::new(cli.location);
    let io = (stdin(), stdout());

    let service = server.serve(io).await?;
    service.waiting().await?;
    Ok(())
}
