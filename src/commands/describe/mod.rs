use clap::Parser;
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
#[clap(name = "describe", about = "Describe a command from the LLM model")]
pub struct Describe {
    // The command to search
    #[clap(help = "The command to describe")]
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

pub async fn handle_describe(describe: Describe) -> Result<(), Box<dyn std::error::Error>> {
    if env::var("OPENAI_API_KEY").is_err() {
        println!("Please set your OpenAI API key using the `set key` command.");
        return Ok(());
    }

    let mut spinner = Spinner::new(
        Spinners::Dots9,
        "Searching description for the command...".into(),
    );

    let model = ModelRef::from_model_name(Model::Gpt35Turbo.to_string());

    let mut option_builder = OptionsBuilder::new();
    option_builder.add_option(llm_chain::options::Opt::Model(model));
    let options = option_builder.build();

    let exec = executor!(chatgpt, options)?;

    let few_shot_examples: Vec<(String, String)> = vec![
        (
            "sudo apt-get".to_string(), 
            "• sudo is used to run a command with elevated rights, typically as the superuser.\n
            • apt-get is the package management command-line tool on Ubuntu and Debian-based systems.\n\t• It is used to interact with the package repositories and install, upgrade, or remove software packages.\n\t• It requires administrative privileges to perform system-wide package operations.".to_string()
        ),
        (
            "hello world".to_string(), 
            "The command \"hello world\" is not a valid shell command.".to_string()
        ),
        (
            "docker build -t myapp:latest --path".to_string(), 
            "• docker build is used to build a Docker image from a Dockerfile.\n\t• -t myapp:latest specifies the name and tag for the resulting image.\n\t\t• myapp is the name of the image.\n\t\t• latest is the tag for the image.\n\t• --path specifies the path to the build context, which is the directory that contains the Dockerfile and any additional files needed for the build.".to_string()
        ),
    ];

    let mut conversation = Conversation::new()
        .with_system(
            "I want you to act as genertaing a description for the command. You should provide a brief explanation of the command and its arguments.".to_string()
        );

    let few_shot_prompt = few_shot_template(few_shot_examples);

    conversation.append(few_shot_prompt);

    let conversation =
        conversation.with_system("Explain the command and its arguments step by step.".to_string());

    let mut chain = Chain::new_with_message_collection(&conversation);

    let step = Step::for_prompt_template(prompt!(
        user: "{{query}}"
    ));
    let parameters = parameters!().with("query", describe.qeury.clone());
    let res = chain.send_message(step, &parameters, &exec).await?;
    let res = res.to_immediate().await?.as_content().to_chat().to_string();
    let res = res.split("Assistant: ").collect::<Vec<&str>>()[1]
        .to_string()
        .trim()
        .to_string();

    spinner.stop_and_persist(
        "✔",
        "Finished searching for the command. Here's the description:".to_string(),
    );

    println!("{}", res);
    Ok(())
}
