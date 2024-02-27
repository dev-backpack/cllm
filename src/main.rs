pub mod commands;

use clap::Parser;
use commands::{Commands, handle_command};

#[derive(Debug, Parser)]
#[clap(
    version,
    author,
)]
struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(_error) = handle_command(cli.commands).await {
    }
}   
