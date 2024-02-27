use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "key", 
    about="Register for an API key to use the OpenAI API",
)]
pub struct Key {
    // The API key to set
    #[clap(help="OpenAI API Key")]
    api_key: String,
}

pub async fn handle_key(key: Key) -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting API Key: {}", key.api_key);
    Ok(())
}