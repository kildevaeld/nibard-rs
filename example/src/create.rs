use clap::{App, Arg};
use futures::TryStreamExt;
use nibard::prelude::*;
use nibard::{Database, Value};

pub fn make() -> App<'static> {
    App::new("create")
}

pub async fn run(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let name = prompt::Input::new("Name").required().build().run()?;

    let mut ctx = db.begin().await?;

    ctx.execute("INSERT INTO todos (label) VALUES (?)", &[Value::Text(name)])
        .await?;

    ctx.commit().await?;

    Ok(())
}
