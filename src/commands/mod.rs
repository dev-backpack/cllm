mod search;
mod set;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Search(search::Search),
    Set(set::Set),
}

pub async fn handle_command(command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::Search(search) => search::handle_search(search).await,
        Commands::Set(set) => set::handle_set(set).await,
    }
}
