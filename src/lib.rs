pub(crate) mod commands;

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

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();

    if let Err(_error) = handle_command(cli.commands).await {
        std::process::exit(1);
    }

    Ok(())
}