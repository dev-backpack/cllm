pub mod commands;

use clap::Parser;
use commands::{handle_command, Commands};

use std::env;

const CLLM: &str = r#"
 _____                                               _____ 
( ___ )---------------------------------------------( ___ )
 |   |                                               |   | 
 |   |                                               |   | 
 |   |       ██████╗██╗     ██╗     ███╗   ███╗      |   | 
 |   |      ██╔════╝██║     ██║     ████╗ ████║      |   | 
 |   |      ██║     ██║     ██║     ██╔████╔██║      |   | 
 |   |      ██║     ██║     ██║     ██║╚██╔╝██║      |   | 
 |   |      ╚██████╗███████╗███████╗██║ ╚═╝ ██║      |   | 
 |   |       ╚═════╝╚══════╝╚══════╝╚═╝     ╚═╝      |   | 
 |   |                                               |   | 
 |___|                                               |___| 
(_____)---------------------------------------------(_____)
"#;

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

    #[arg(short, long)]
    pub vers: bool,
}

#[tokio::main]
async fn main() {
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
    if env::args().collect::<Vec<String>>().len() == 1 {
        println!(
            "{}\n Welcome to CLLM! Try running `cllm --help` to get started.\n",
            CLLM
        );
        return;
    }

    // Parse the command line arguments
    let cli = Cli::parse();

    if let Err(_error) = handle_command(cli.commands).await {
        println!("\n\nError Occurred\n{}", _error.to_string());
    }
}
