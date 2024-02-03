import typer
import set.index

app = typer.Typer()

app.add_typer(set.index.set_app, name="set")

if __name__ == "__main__":
    app()
