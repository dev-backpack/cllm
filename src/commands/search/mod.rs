use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "search", 
    about="Search a command from the LLM model",
)]
pub struct Search {
    // The command to search
    #[clap(help="The command to search")]
    qeury: String,    
}

pub async fn handle_search(search: Search) -> Result<(), Box<dyn std::error::Error>> {
    println!("Searching for: {}", search.qeury);
    Ok(())
}