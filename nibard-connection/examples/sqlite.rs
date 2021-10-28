use nibard_connection::{Database, DatabaseRow, Executor, Row, RowExt};
use nibard_shared::Type;

const CREATE: &str = r#"
CREATE TABLE IF NOT EXISTS todos(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    label TEXT NOT NULL,
    description TEXT DEFAULT NULL,
    completed DATETIME DEFAULT CURRENT_TIMESTAMP,
    started BOOLEAN NOT NULL DEFAULT false
);
"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = Database::open("sqlite::memory:").await?;

    db.execute(CREATE).await?;

    db.execute("insert into todos (label) values('Hello')")
        .await?;

    let stream = db.fetch_one("select * from todos").await?;

    println!("STREAM {:?}", stream.try_get("started", Some(Type::Bool)));

    Ok(())
}
