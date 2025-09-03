use crate::tool::loc::Location;
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(long, value_name = "LISTEN_ADDRESS")]
    pub http_listen: Option<String>,

    #[arg(long, value_name = "LOCATION")]
    pub location: Option<Location>,
}
