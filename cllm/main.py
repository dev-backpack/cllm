import typer
from .set import index as set_index
import os
import json
import platform
from langchain.chains import LLMChain
from langchain_openai import ChatOpenAI
from langchain_core.prompts import (ChatPromptTemplate, FewShotChatMessagePromptTemplate)

app = typer.Typer(help="Empower your CLI experience with a command search tool driven by LLM magic!")

app.add_typer(set_index.set_app, name="set", help="Set up configurations used in cllm")

@app.command()
def search(query : str):
    """Search a command from the LLM model"""
    home_dir = os.path.expanduser("~")
    save_dir = os.path.join(home_dir, ".cllm")
    
    if not os.path.exists(save_dir):
        typer.echo("Please set the API key first.")
        raise typer.Exit(code=1)
    
    filepath = os.path.join(save_dir, "credentials.json")
    
    with open(filepath, "r", encoding="utf-8") as file:
        data = json.load(file)
        api_key = data["OPEN_AI"]

    os.environ["OPENAI_API_KEY"] = api_key

    examples = [
        {"input" : "Show all pods in k8s", 
         "output" : "kubectl get pods"},
        {"input" : "Find all files recursively within the current directory that contain 'a' in their filenames.", 
         "output" : "find . -type f -name '*a*' -print"},
        {"input" : "Provide the command to build and push a Docker image from the current directory.", 
         "output" : "docker build -t myapp:latest . -—push"},
    ]

    example_prompt = ChatPromptTemplate.from_messages(
        [
            ("human", "{input}"),
            ("ai", "{output}"),
        ]
    )

    chat_template = ChatPromptTemplate.from_messages(
        [
            ("system", "I want you to act as generating a command for request tasks on {os_name}. \
             Also please don't explain the commands, just generate the command."),
            FewShotChatMessagePromptTemplate(example_prompt=example_prompt, examples=examples).format(),
            ("human", "task : {task}"),

        ]
    )

    prompt = chat_template.format_messages(
        task = query,
        os_name = platform.system()
    )

    model = ChatOpenAI()
    
    output = model.invoke(prompt)
    
    typer.echo(output.content)

if __name__ == "__main__":
    app()
