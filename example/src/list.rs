use clap::App;
use futures::{pin_mut, TryStreamExt};
use nibard::prelude::*;
use nibard::Database;
use nibard::Dialect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Todo {
    id: i64,
    label: String,
}

pub fn make() -> App<'static> {
    App::new("list")
}

pub async fn run(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    // let stream = "todos"
    //     .select("todos.*")
    //     .to_query(Dialect::Sqlite)
    //     .fetch(db);

    let stream = db.query("todos".select("todos.*"));

    pin_mut!(stream);

    while let Ok(Some(next)) = stream.try_next().await {
        println!("{:?}", next.try_into::<Todo>()?)
    }

    Ok(())
}
