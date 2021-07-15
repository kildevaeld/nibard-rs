mod create;
mod list;

use clap::App;
use futures::TryStreamExt;
use nibard::{Database, Executor};

async fn create_schema(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = db.begin().await?;

    let scheme = include_str!("./schema.sql");

    ctx.execute_many(scheme)
        .await
        .try_collect::<Vec<_>>()
        .await?;

    ctx.commit().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::open("sqlite:./todos.sqlite").await?;

    create_schema(&db).await?;

    let app = App::new("todos")
        .subcommand(list::make())
        .subcommand(create::make())
        .get_matches();

    if let Some(matches) = app.subcommand_matches("list") {
        list::run(&db).await?;
    } else if let Some(matches) = app.subcommand_matches("create") {
        create::run(&db).await?;
    }

    Ok(())
}
