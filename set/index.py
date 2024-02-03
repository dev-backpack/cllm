import typer
import os
import json
from typing_extensions import Annotated

set_app = typer.Typer()

@set_app.command("key")
def set_key(api_key: Annotated[str, typer.Argument(help="OpenAI API Key")]):
    """Register for an API key to use the OpenAI API"""
    home_dir = os.path.expanduser("~")
    save_dir = os.path.join(home_dir, ".cllm")
    
    if not os.path.exists(save_dir):
        os.makedirs(save_dir)
    
    filepath = os.path.join(save_dir, "credentials.json")


    data = {
        "OPEN_AI": api_key
    }
    
    with open(filepath, "w", encoding="utf-8") as file:
        json.dump(data, file)

    typer.echo("Saving completed.")


if __name__ == "__main__":
    set_app()
