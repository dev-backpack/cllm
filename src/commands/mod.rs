mod describe;
mod history;
mod search;
mod set;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Search(search::Search),
    Set(set::Set),
    History(history::History),
    Describe(describe::Describe),
}

pub async fn handle_command(command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::Search(search) => search::handle_search(search).await,
        Commands::Set(set) => set::handle_set(set).await,
        Commands::History(history) => history::handle_history(history).await,
        Commands::Describe(describe) => describe::handle_describe(describe).await,
    }
}
