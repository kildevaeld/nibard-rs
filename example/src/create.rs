use clap::{App, Arg};
use futures::TryStreamExt;
use nibard::prelude::*;
use nibard::{Database, Value};

pub fn make() -> App<'static> {
    App::new("create")
}

fn test(db: Database, name: String) -> impl std::future::Future<Output = ()> + 'static + Send {
    Box::pin(async move {
        let mut ctx = db.begin().await.unwrap();

        ctx.execute((
            "INSERT INTO todos (label) VALUES (?)",
            &[Value::Text(name)][..],
        ))
        .await
        .unwrap();

        ctx.commit().await.unwrap();
    })
}

pub async fn run(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let name = prompt::Input::new("Name").required().build().run()?;

    // let mut ctx = db.begin().await?;

    // ctx.execute((
    //     "INSERT INTO todos (label) VALUES (?)",
    //     &[Value::Text(name)][..],
    // ))
    // .await?;
    tokio::spawn(test(db.clone(), name)).await?;

    // ctx.commit().await?;

    Ok(())
}
