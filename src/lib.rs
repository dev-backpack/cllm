pub(crate) mod commands;

use clap::Parser;
use commands::{handle_command, Commands};

use std::env;

#[derive(Debug, Parser)]
#[clap(
    version,
    about = "Empower your CLI experience with a command search tool driven by LLM magic!\n\
    If you have any questions or suggestions, feel free to open an issue on the github repo.\n\
    GitHub: https://github.com/dev-backpack/cllm"
)]
struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Set the OPENAI_API_KEY environment variable
    let home_dir = dirs::home_dir().unwrap();
    let save_dir = home_dir.join(".cllm");
    let config_path = save_dir.join("credentials.json");

    if config_path.exists() {
        let config = std::fs::read_to_string(config_path).unwrap();
        let config: serde_json::Value = serde_json::from_str(&config).unwrap();

        if config["OPEN_AI"].is_string() {
            let api_key = config["OPEN_AI"].as_str().unwrap();
            env::set_var("OPENAI_API_KEY", api_key);
        }
    }

    let cli: Cli = Cli::parse();

    if let Err(_error) = handle_command(cli.commands).await {
        std::process::exit(1);
    }

    Ok(())
}
