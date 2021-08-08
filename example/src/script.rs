use clap::{App, Arg, ArgMatches};
use futures::{pin_mut, TryStreamExt};
use mlua::{chunk, Lua, MetaMethod, UserData};
use nibard::prelude::*;
use nibard::Database;
use nibard::Dialect;
use nibard_lua::LuaDatabase;

pub fn make() -> App<'static> {
    App::new("script").arg(Arg::new("path").takes_value(true))
}

pub async fn run(db: &Database, args: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let lua = Lua::new();

    let db = LuaDatabase::new(db.clone());

    lua.load(chunk! {


        local iter = $db:fetch("SELECT * FROM todos")
        for row in iter do
            print(row.label)
        end


    })
    .exec_async()
    .await?;

    Ok(())
}
