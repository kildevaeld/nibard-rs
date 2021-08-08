use async_stream::stream;
use futures::{pin_mut, Stream, StreamExt};
use mlua::{
    serde::LuaSerdeExt, Error as LuaError, Lua, UserData, UserDataFields, UserDataMethods,
    Value as LuaValue,
};
use nibard::{connection::Error as NibardError, Database, DatabaseRow, Executor, RowExt};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct LuaDatabase {
    db: Database,
}

impl LuaDatabase {
    pub fn new(db: Database) -> LuaDatabase {
        LuaDatabase { db }
    }
}

pub struct CollectionIter<S> {
    stream: Arc<Mutex<S>>,
}

impl<S> Clone for CollectionIter<S> {
    fn clone(&self) -> Self {
        CollectionIter {
            stream: self.stream.clone(),
        }
    }
}

impl<S> CollectionIter<S>
where
    S: Stream<Item = Result<DatabaseRow, NibardError>> + std::marker::Unpin,
{
    pub async fn next(&self) -> Option<Result<DatabaseRow, NibardError>> {
        let mut stream = self.stream.lock().await;

        match (&mut *stream).next().await {
            Some(s) => match s {
                Ok(s) => Some(Ok(s)),
                Err(e) => Some(Err(e)),
            },
            None => None,
        }
    }
}

impl UserData for LuaDatabase {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("dialect", |_, this| Ok(this.db.dialect().to_string()))
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("fetch_one", |lua, this, params: (String,)| async move {
            //
            let row = this
                .db
                .fetch_one(params.0.as_str())
                .await
                .map_err(LuaError::external)?
                .to_map();

            lua.to_value(&row)
        });

        methods.add_async_method(
            "fetch",
            |lua: &Lua, this: LuaDatabase, params: (String,)| async move {
                let stream = Box::pin(stream!({
                    let stream = this.db.fetch(params.0.as_str());
                    pin_mut!(stream);
                    while let Some(next) = stream.next().await {
                        yield next;
                    }
                }))
                .fuse();

                let stream = CollectionIter {
                    stream: Arc::new(Mutex::new(stream)),
                };

                lua.create_async_function(move |lua, ()| {
                    let stream = stream.clone();
                    async move {
                        let row = match stream.next().await {
                            Some(Ok(row)) => row.to_map(),
                            Some(Err(err)) => return Err(LuaError::external(err)),
                            None => return Ok(LuaValue::Nil),
                        };
                        let value = lua.to_value(&row)?;
                        Ok(value)
                    }
                })
            },
        );
    }
}
