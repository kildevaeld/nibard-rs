use clap::App;
use futures::TryStreamExt;
use nibard::prelude::*;
use nibard::Database;
use nibard::Dialect;

pub fn make() -> App<'static> {
    App::new("list")
}

pub async fn run(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let query = "todos"
        .select(("id", "label", "description"))
        .to_query(Dialect::Sqlite);
    let mut stream = query.fetch(db);

    while let Ok(Some(next)) = stream.try_next().await {
        println!("{:?}", next.try_get("label")?)
    }

    Ok(())
}
