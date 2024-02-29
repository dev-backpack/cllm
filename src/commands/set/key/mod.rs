use clap::Parser;

use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Parser)]
#[clap(name = "key", about = "Register for an API key to use the OpenAI API")]
pub struct Key {
    // The API key to set
    #[clap(help = "OpenAI API Key")]
    api_key: String,
}

pub async fn handle_key(key: Key) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().unwrap();
    let save_dir = home_dir.join(".cllm");
    let config_path = save_dir.join("credentials.json");

    if !save_dir.exists() {
        std::fs::create_dir_all(&save_dir)?;
    }

    let mut config = if config_path.exists() {
        let config = std::fs::read_to_string(config_path.clone())?;
        serde_json::from_str(&config)?
    } else {
        serde_json::json!({})
    };

    config["OPEN_AI"] = key.api_key.clone().into();
    let config = serde_json::to_string_pretty(&config)?;
    File::create(config_path)?.write_all(config.as_bytes())?;
    env::set_var("OPENAI_API_KEY", key.api_key);

    println!("API key set successfully.");
    Ok(())
}
