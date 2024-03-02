use clap::Args;
use sqlite::{Connection, State, Value};
use tabled::{
    settings::{peaker::PriorityMax, Width},
    Table, Tabled,
};

#[derive(Debug, Args)]
#[clap(name = "history", about = "Display the previous search history")]
pub struct History {
    #[clap(short, long, default_value = "10")]
    limit: i64,

    #[clap(short, long)]
    query: Option<String>,
}

#[derive(Tabled)]
pub struct HistoryRow {
    id: i64,
    input: String,
    output: String,
    created_at: String,
}

pub async fn connect_history() -> Connection {
    let home_dir = dirs::home_dir().unwrap();
    let save_dir = home_dir.join(".cllm");
    let db_path = save_dir.join(":memory:");

    let connection = sqlite::open(db_path).unwrap();

    connection
        .execute(
            "
        CREATE TABLE IF NOT EXISTS history (
            id INTEGER PRIMARY KEY,
            input TEXT NOT NULL,
            output TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        ",
        )
        .unwrap();

    connection
}

pub async fn insert_history(
    input: String,
    output: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let connection = connect_history().await;

    let query = "INSERT INTO history (input, output) VALUES (:input, :output)";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind_iter::<_, (_, Value)>([(":input", input.into()), (":output", output.into())])?;

    statement.next().unwrap();

    Ok(())
}

pub async fn handle_history(history: History) -> Result<(), Box<dyn std::error::Error>> {
    let limit = history.limit;
    let condition = format!("%{}%", history.query.unwrap_or("".to_string())).to_string();

    let connection = connect_history().await;
    let mut rows: Vec<HistoryRow> = Vec::new();

    let query =
        "SELECT * FROM history WHERE input LIKE :query ORDER BY created_at DESC LIMIT :limit";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind_iter::<_, (_, Value)>([
        (":query", condition.into()),
        (":limit", limit.to_string().into()),
    ])?;

    while let Ok(State::Row) = statement.next() {
        let id: i64 = statement.read(0)?;
        let input: String = statement.read(1)?;
        let output: String = statement.read(2)?;
        let created_at: String = statement.read(3)?;

        rows.push(HistoryRow {
            id,
            input,
            output,
            created_at,
        });
    }

    let terminal_size::Width(width) = terminal_size::terminal_size().unwrap().0;
    let mut table = Table::new(rows);
    table.with(Width::wrap(width as usize).priority::<PriorityMax>());

    println!("{}", table);

    Ok(())
}
