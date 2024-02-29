use clap::Parser;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use llm_chain::{
    chains::conversation::Chain,
    executor,
    options::{ModelRef, OptionsBuilder},
    parameters, prompt,
    prompt::{ChatMessageCollection, Conversation},
    step::Step,
};
use llm_chain_openai::chatgpt::Model;
use spinners::{Spinner, Spinners};
use std::env;
#[derive(Debug, Parser)]
#[clap(name = "search", about = "Search a command from the LLM model")]
pub struct Search {
    // The command to search
    #[clap(help = "The command to search")]
    qeury: String,
}

pub fn few_shot_template(list: Vec<(String, String)>) -> ChatMessageCollection<String> {
    let mut ret_prompt = Conversation::new();

    for (user, assistant) in &list {
        ret_prompt = ret_prompt
            .with_user(user.to_string())
            .with_assistant(assistant.to_string());
    }

    ret_prompt
}

pub async fn handle_search(search: Search) -> Result<(), Box<dyn std::error::Error>> {
    if !env::var("OPENAI_API_KEY").is_ok() {
        println!("Please set your OpenAI API key using the `set key` command.");
        return Ok(());
    }

    let mut spinner = Spinner::new(Spinners::Dots9, "Searching for the command...".into());

    let model = ModelRef::from_model_name(Model::Gpt35Turbo.to_string());

    let mut option_builder = OptionsBuilder::new();
    option_builder.add_option(llm_chain::options::Opt::Model(model));
    let options = option_builder.build();

    let exec = executor!(chatgpt, options)?;

    let few_shot_examples: Vec<(String, String)> = vec![
        ("Show all pods in k8s".to_string(), "kubectl get pods".to_string()),
        ("Find all files recursively within the current directory that contain 'a' in their filenames.".to_string(), "find . -type f  -name '*a*' -print".to_string()),
        ("Provide the command to build and push a Docker image from the current directory.".to_string(), "docker build -t myapp:latest --path".to_string()),
    ];

    let mut conversation = Conversation::new()
        .with_system_template(
            "I want you to act as generating a command for request tasks on {{os_name}}. Also please don't explain the commands, just generate the command.",
            &parameters!{"os_name" => env::consts::OS}
        ).unwrap();

    let few_shot_prompt = few_shot_template(few_shot_examples);

    conversation.append(few_shot_prompt);

    let conversation =
        conversation.with_system("Only generate the command, don't explain it".to_string());

    let mut chain = Chain::new_with_message_collection(&conversation);

    let step = Step::for_prompt_template(prompt!(
        user: "task : {{query}}"
    ));
    let parameters = parameters!().with("query", search.qeury);
    let res = chain.send_message(step, &parameters, &exec).await?;
    let res = res.to_immediate().await?.as_content().to_chat().to_string();
    let res = res.split("Assistant: ").collect::<Vec<&str>>()[1]
        .to_string()
        .trim()
        .to_string();

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(res.clone().to_string()).unwrap();

    spinner.stop_and_persist(
        "✔",
        "Finished searching for the command and copied to your clipboard :)".into(),
    );

    println!("{}", res);
    Ok(())
}
