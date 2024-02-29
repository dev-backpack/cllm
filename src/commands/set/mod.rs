mod key;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
#[clap(name = "set", about = "Configure application resources")]
pub enum Commands {
    // Set the API key
    Key(key::Key),
}

#[derive(Debug, Parser)]
pub struct Set {
    #[clap(subcommand)]
    subcmd: Commands,
}

pub async fn handle_set(set: Set) -> Result<(), Box<dyn std::error::Error>> {
    match set.subcmd {
        Commands::Key(key) => key::handle_key(key).await,
    }
}
